import requests
import os
import sys
from bs4 import BeautifulSoup

if len(sys.argv) != 3:
    print("Usage: python fetchInputs.py year day")
    exit(1)

year = sys.argv[1]
day = sys.argv[2]

file = "inputs/day{day}-input.txt".format(day=day)


def fetchPersonalInput(session, year, day):
    url = "https://adventofcode.com/{year}/day/{day}/input".format(year=year,
                                                                   day=day)

    r = requests.get(url, cookies={"session": session})
    if r.status_code != 200:
        print("Failed to fetch input Status code: {code}".format(
            code=r.status_code))
        exit(1)

    with open(file, "w") as f:
        f.write(r.text.strip())


def getExemples(session, year, day):
    if not os.path.exists("examples"):
        os.mkdir("examples".format(day=day))
    if not os.path.exists("examples/day{day}".format(day=day)):
        os.mkdir("examples/day{day}".format(day=day))

    url = "https://adventofcode.com/{year}/day/{day}".format(year=year,
                                                             day=day)

    r = requests.get(url, cookies={"session": session})
    if r.status_code != 200:
        print("Failed to fetch input")
        exit(1)

    soup = BeautifulSoup(r.text, 'html.parser')

    #finding the div with the id
    i = 0
    codeSections = soup.find_all('code')
    for code in codeSections:
        codePath = "examples/day{day}/example{i}.txt".format(day=day, i=i)
        i += 1
        with open(codePath, "w") as f:
            f.write(code.text.strip())


session = os.environ.get("AOC_SESSION")
if session is None:
    print("No session found")
    exit(1)

if not os.path.exists(file):
    fetchPersonalInput(session, year, day)

getExemples(session, year, day)
