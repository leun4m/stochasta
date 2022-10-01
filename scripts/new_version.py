#!/usr/bin python3

# Script to create a new version.
# The env must provide a GITHUB_API_TOKEN.
#
# - sets version in Cargo.toml
# - sets version in CHANGELOG.md
# - commits those files
# - creates annoted version tag
# - publishes the new release on GitHub
#   (with the RELEASE NOTES from CHANGELOG)


import os
import datetime
import requests
from dotenv import load_dotenv


def main():
    check_secure_for_release()
    version = input("Version: ").strip()
    date = datetime.datetime.now().date().isoformat()

    release_notes = get_release_notes()
    update_cargo(version)
    update_changelog(version, date)
    os.system("git diff")

    accept = input("Commit? [y/N]")
    if accept.lower() == "y":
        os.system(f"git add Cargo.toml CHANGELOG.md")
        os.system(f"git commit -m 'Update version'")
        os.system(f"git tag -a v{version} -m ''")
        os.system(f"git push --follow-tags")

    accept = input("Publish? [y/N]")
    if accept.lower() == "y":
        publish_release(f"v{version}", release_notes)
        os.system(f"rm RELEASE_NOTES")
        os.system(f"git checkout release")
        os.system(f"git merge v{version} --ff-only")
        os.system(f"git push")
        os.system(f"git checkout develop")


def check_secure_for_release():
    branch = os.popen("git branch --show-current").read()
    if branch.strip() != "develop":
        abort("Not on branch develop")
    changes = os.popen("git status --short").read()
    if changes.strip() != "":
        abort("Repository not clean")


def abort(message):
    print(f"ERROR: {message}!")
    exit()


def get_release_notes():
    with open("CHANGELOG.md", "r", encoding="utf-8") as file:
        data = file.read()
        notes = data.split("\n## Unreleased\n")[1].split("\n## ")[0].strip()
        notes += "\n\n---\n\n"
        notes += "*For a complete history see: [CHANGELOG.md](./CHANGELOG.md)*"

    with open("RELEASE_NOTES", "w", encoding="utf-8") as file:
        file.write(notes)

    return notes


def update_cargo(version):
    with open("Cargo.toml", "r", encoding="utf-8") as file:
        data = file.readlines()
        if data[2].startswith("version"):
            data[2] = f"version = \"{version}\"\n"

    with open("Cargo.toml", "w", encoding="utf-8") as file:
        file.writelines(data)


def update_changelog(version, date):
    with open("CHANGELOG.md", "r", encoding="utf-8") as file:
        data = file.read()
        data = data.replace("\n## Unreleased\n",
                            f"\n## Unreleased\n\n## [{version}] ({date})\n")
        data += f"[{version}]: https://github.com/leun4m/stochasta/releases/tag/v{version}\n"

    with open("CHANGELOG.md", "w", encoding="utf-8") as file:
        file.write(data)


def publish_release(version, release_notes):
    owner = "leun4m"
    repo = "stochasta"
    url = f"https://api.github.com/repos/{owner}/{repo}/releases"
    obj = {
        "owner": owner,
        "repo": repo,
        "tag_name": version,
        #   "target_commitish": 'develop',
        "name": version,
        "body": release_notes,
        "draft": False,
        "prerelease": False,
        "generate_release_notes": False
    }
    token = os.getenv("GITHUB_API_TOKEN")
    requests.post(url, json=obj, headers={
        "Accept": "application/vnd.github+json",
        "Authorization": f"Bearer {token}"
    })


if __name__ == "__main__":
    load_dotenv()
    main()
