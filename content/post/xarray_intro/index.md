---
title: Python, Jupyter Notebooks, and xarray introduction
subtitle: 

# Summary for listings and search engines
summary: Python is a fantastic tool for handling meteorological and climate model output. Here, I go over the basics of climate analysis workflows in Python, with a twist at the end. 

# Link this post with a project
projects: []

share: false

# Date published
date: '2021-09-09T00:00:00Z'

# Date updated
lastmod: '2021-09-09T00:00:00Z'

# Is this an unpublished draft?
draft: false

# Show this page in the Featured widget?
featured: false

authors:
  - admin

tags:
  - Python
  - xarray
  - Jupyter
  - Binder

categories:
  - Python
  - xarray
  - Jupyter
  - Binder
---

During my dissertation research, I began playing with climate model output, which ended up integral in two of my three chapters. None of the work in the dissertation would have been possible without tools such as [xarray](https://docs.xarray.dev/en/stable/index.html) which made working with climate model data as painless as can possibly be. I'm writing this blog post in 2022, and I originally wrote this script for a lab meeting in 2021 meant to demonstrate why Jupyter Notebooks are so great, however, I think the introduction to netCDFs (the default climate model format) and `xarray` are the real strengths of the code below. My lab at the time was invested in the R ecosystem, so the goal of the script below was to demonstrate how to leverage the strengths of two coding languages within the same script. If you follow along with the script below and have an introductory understanding of R and Python here's what you'll end up learning:

* netCDF file structure
* Opening and manipulating netCDF files in Python using `xarray`
* Plot data from netCDF files in Python using `matplotlib`
* Pass data from `pandas` in Python to R
* Plot using `ggplot2` in R

If following along with the script isn't your cup of tea, feel free to open the script and actually run the code through Binder! Just click on the icon below and wait a few minutes for the Binder environment to build. From there, you'll be able to run and edit the code for yourself in JupyterLab! If you're unfamiliar with JupyterLab, wait for an upcoming blog post that introduces JupyterLab and how to use it for scientific computing - I'll be sure to link it here once it's up.

<script src="https://gist.github.com/davidfastovich/93b1b569cb8fd8433d9d80d9ff8725ea.js"></script>

[![Binder](https://mybinder.org/badge_logo.svg)](https://mybinder.org/v2/gh/davidfastovich/Python-Jupyter-Tutorial/c3aba8a7a98825e5112bebdf87c0f1cb304d6cf0?urlpath=lab%2Ftree%2FR%2BPython.ipynb)