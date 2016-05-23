from setuptools import setup, find_packages

setup(
    name='imgsize',
    author='Jonas Obrist',
    license='BSD',
    version='1.0',
    test_suite='imgsize.tests',
    packages=find_packages(),
    classifiers=[
        'Development Status :: 5 - Production/Stable',
        'Intended Audience :: Developers',
        'License :: OSI Approved :: BSD License',
        'Operating System :: OS Independent',
        'Programming Language :: Python :: 2.7',
        'Programming Language :: Python :: 3.4',
        'Programming Language :: Python :: 3.5'
    ]
)

