from setuptools import setup, find_packages
from subprocess import Popen, PIPE
from setuptools.command.install import install
import re
import subprocess

def get_version():
    """
    Returns project version as string from 'git describe' command.
    """
    pipe = Popen('git describe --tags --always', stdout=PIPE, shell=True)
    version = str(pipe.communicate()[0].rstrip().decode("utf-8"))
    return str(re.sub(r'-g\w+', '', version))

setup(
    name="recursive_matching",
    version=get_version(),
    packages=find_packages(),
    install_requires=[
        'numpy'
    ],
)