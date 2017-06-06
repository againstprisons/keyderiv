from setuptools import setup, find_packages

install_requires = [
    "Flask",
    "gunicorn",
    "pynacl"
]

# get our version
with open("earmms_keyderiv/version.py") as fh:
    data = fh.read()

    g = {}
    l = {}

    exec(data, g, l)

    version = l["version"]

setup_args = {
    "name": "earmms_keyderiv",
    "packages": find_packages(),
    "version": version,
    "description": "Key derivation microservice for EARMMS",
    "install_requires": install_requires,
    "author": "Alice Jenkinson",
    "author_email": "h@nea.nz",
    "license": "MIT"
}

setup(**setup_args)
