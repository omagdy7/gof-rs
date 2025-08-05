#!/usr/bin/env python3
import re
import json

def parse_patterns(file_content):
    # Split the text into sections by the pattern delimiter
    sections = file_content.split("\n\n")
    print(sections)

    patterns = []
    for section in sections:
        # Extract the pattern name
        name_match = re.match(r":([\w\-\(\),/ ]+):", section)
        if not name_match:
            continue
        name = name_match.group(1).strip()


        # Extract the pattern description
        pattern_match = re.search(r"\n\t.*([\.\*]*\n)", section)
        pattern = pattern_match.group(0).strip() if pattern_match else None

        # Extract the discoverer
        discoverer_match = re.search(r"Found by ([\w\s]+) in", section, re.IGNORECASE)
        discoverer = discoverer_match.group(1).strip() if discoverer_match else None

        # Add the extracted data to the patterns list
        patterns.append({
            "name": name,
            "pattern": pattern,
            "discoverer": discoverer
        })

    return patterns

# Load the input file
with open("../lexicon_stripped.txt", "r") as file:
    content = file.read()

# Parse the patterns
parsed_patterns = parse_patterns(content)

print(parsed_patterns[0])

# Output the result as JSON
output_file = "patterns.json"
with open(output_file, "w") as json_file:
    json.dump(parsed_patterns, json_file, indent=4)




print(f"Patterns saved to {output_file}")

