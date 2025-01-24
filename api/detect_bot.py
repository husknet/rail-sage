from flask import Flask, request, jsonify
import re
from ipwhois import IPWhois
from collections import defaultdict
from datetime import datetime, timedelta
import geoip2.database

app = Flask(__name__)

# Path to the GeoLite2-Country.mmdb file
GEOIP_DB_PATH = "static/GeoLite2-Country.mmdb"

# List of known scraper ISPs
SCRAPER_ISPS = [
    "Microsoft Corporation", "Netcraft", "DigitalOcean", "Amazon Technologies Inc.",
    "Google LLC", "Linode, LLC", "OVH SAS", "Hetzner Online GmbH",
    "Alibaba", "Oracle Corporation", "SoftLayer Technologies", "Fastly",
    "Cloudflare", "Akamai Technologies", "Hurricane Electric", "Hostwinds",
    "Choopa", "Contabo GmbH", "Leaseweb", "Scaleway", "Vultr", "Ubiquity"
]

# Traffic data for IP-based load tracking
TRAFFIC_DATA = defaultdict(list)  # Stores request timestamps for each IP

# Thresholds for traffic load analysis
REQUEST_THRESHOLD = 10  # Max requests allowed in the given timeframe
TIMEFRAME = timedelta(seconds=30)  # Timeframe to monitor traffic

def check_scraper_isp(ip: str) -> bool:
    """
    Checks if the user's IP belongs to a known scraper ISP.
    """
    try:
        obj = IPWhois(ip)
        details = obj.lookup_rdap()
        isp = details.get("network", {}).get("name", "")
        if any(scraper in isp for scraper in SCRAPER_ISPS):
            return True
    except Exception as e:
        print(f"Error during ISP lookup: {e}")
    return False

def is_traffic_suspicious(ip: str) -> bool:
    """
    Analyzes traffic from the given IP to detect suspicious activity.
    """
    now = datetime.now()
    TRAFFIC_DATA[ip].append(now)
    TRAFFIC_DATA[ip] = [ts for ts in TRAFFIC_DATA[ip] if now - ts <= TIMEFRAME]
    return len(TRAFFIC_DATA[ip]) > REQUEST_THRESHOLD

def get_country_from_ip(ip: str) -> str:
    """
    Gets the country of the user from their IP address using GeoLite2-Country.
    """
    try:
        with geoip2.database.Reader(GEOIP_DB_PATH) as reader:
            response = reader.country(ip)
            return response.country.name or "Unknown"
    except Exception as e:
        print(f"Error during GeoIP lookup: {e}")
        return "Unknown"

@app.route('/api/detect_bot', methods=['POST'])
def detect_bot():
    data = request.json
    user_agent = data.get('user_agent', '')
    ip = data.get('ip', '')

    # Check if the user agent indicates a bot
    bot_patterns = [r'bot', r'scraper', r'crawl', r'spider']
    is_bot = any(re.search(pattern, user_agent.lower()) for pattern in bot_patterns)

    # Check if the ISP belongs to a known scraper
    is_scraper_isp = check_scraper_isp(ip)

    # Check if the traffic from this IP is suspicious
    is_suspicious_traffic = is_traffic_suspicious(ip)

    # Get the country from the user's IP
    country = get_country_from_ip(ip)

    # Final decision
    is_bot_final = is_bot or is_scraper_isp or is_suspicious_traffic

    return jsonify({
        'is_bot': is_bot_final,
        'country': country,
        'details': {
            'bot_user_agent': is_bot,
            'scraper_isp': is_scraper_isp,
            'suspicious_traffic': is_suspicious_traffic
        }
    })

# Vercel will call this handler
def handler(event, context):
    return app(event, context)
