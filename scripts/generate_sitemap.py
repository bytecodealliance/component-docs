import os
from urllib.parse import urljoin
from datetime import datetime
from pathlib import Path

import argparse

def parse_summary(summary_file_path):
    """Parse URLs from the SUMMARY.md file."""
    with open(summary_file_path, "r") as file:
        for line in file:
            if "](" in line:
                url = line.split("](")[1].split(")")[0]
                # Add .html extension if not the root URL
                if url.endswith(".md"):
                    url = url[:-3] + ".html"
                yield url

def determine_priority(url_path, higher_priority_section):
    """Determine the priority based on the URL path and specified higher priority section."""
    if url_path.count("/") <= 1:  # Pages directly under the base URL
        return "1.0"
    elif higher_priority_section and url_path.startswith(f"./{higher_priority_section}"):  # Pages in the specified higher priority section
        return "0.8"
    else:
        return "0.5"  # All other pages

def generate_sitemap(domain, output_path, summary_file_path, higher_priority_section):
    """Generate a sitemap XML file from SUMMARY.md structure."""
    domain = "https://" + domain
    urls = parse_summary(summary_file_path)  # Add base URL to the list of URLs
    urls = [""] + list(urls)

    sitemap = '<?xml version="1.0" encoding="UTF-8"?>\n'
    sitemap += '<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">\n'

    for url in urls:
        full_url = urljoin(domain, url)
        priority = determine_priority(url, higher_priority_section)

        sitemap += "  <url>\n"
        sitemap += f"    <loc>{full_url}</loc>\n"
        sitemap += "    <changefreq>weekly</changefreq>\n"
        sitemap += f"    <priority>{priority}</priority>\n"
        sitemap += "  </url>\n"

    sitemap += "</urlset>"

    # Write the sitemap to the specified output path
    with open(output_path, "w") as file:
        file.write(sitemap)

DEFAULT_SUMMARY_MD_PATH = (Path(__file__).parent / "../component-model/src/SUMMARY.md").resolve()
DEFAULT_SITEMAP_XML_PATH = (Path(__file__).parent / "../component-model/book/html/sitemap.sml").resolve()

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Generate a sitemap for mdBook")
    parser.add_argument("-d", "--domain", required=True, help="Domain for the mdBook site (e.g., 'component-model.bytecodealliance.org')")
    parser.add_argument("-o", "--output-path", default=DEFAULT_SITEMAP_XML_PATH, help="Output path for the sitemap file")
    parser.add_argument("-s", "--summary-md-path", default=DEFAULT_SUMMARY_MD_PATH, help="Path to SUMMARY.md")
    parser.add_argument("-p", "--higher-priority", help="Subsection path (e.g., 'design') to assign a higher priority of 0.8")
    args = parser.parse_args()

    summary_file_path = Path(args.summary_md_path).resolve()
    if not summary_file_path.exists():
        raise FileNotFoundError(f"failed to find summary file [{summary_file_path}]")

    output_path = Path(args.output_path).resolve()
    if not output_path.parent.exists():
        raise FileNotFoundError(f"failed to find output dir [{output_path.parent}]")

    generate_sitemap(args.domain, output_path, summary_file_path, args.higher_priority)
