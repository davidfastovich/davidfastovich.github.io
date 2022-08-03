---
title: Using R in Jupyter Notebooks and Binder
subtitle: 

# Summary for listings and search engines
summary: Reproducible science is best facilitated by open science. Using open source tools like R is great and can be strengthened by writing and sharing reproducible code through GitHub and Binder.

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
  - R
  - neotoma
  - Jupyter
  - Binder

categories:
  - R
  - neotoma
  - Jupyter
  - Binder
---

Making research data and code publicly available is core to making science more transparent and reproducible, yet more can be done towards this goal. Researchers commonly share their code and data through various academic repositories like [Zenodo](https://zenodo.org/) or public code repositories like [GitHub](https://github.com/), however, there remains a limitation to easy reproducibility: creating the computing environment that produced the code. The computing environment encompasses everything about the computer that was used to produce the final analyses and figures from the operating system to the coding language version and packages used for analyses. This is where [Binder](https://mybinder.org/) comes in - this is a **free** service that creates a virtual computer in the cloud based on the specifications listed in a GitHub repository. In the process of building the environment, Binder also clones the GitHub repository making it painless to reproduce analyses. **From an end user's perspective, this means that GitHub repositories that link to Binder environments allow someone to view the code, click a single button, and within minutes begin running that code on a computer in the cloud.** Binder is particularly well suited for scientific computing done using [R](https://cran.r-project.org/) and [Python](https://www.python.org/) because these code languages are freely accessible.

This introduction will teach you how to build an R Binder environment that can be used through [Jupyter Notebooks](https://jupyter.org/) or [RStudio](https://www.rstudio.com/). This guide assumes that you have some experience with GitHub and are able to build your own public code repository. Once you have you're GitHub repository live (we'll use this introduction to the [Neotoma Paleoecology Database](https://www.neotomadb.org/) I wrote some time back as an example), you need to a couple of files to the repository. First, you need to add a file named `runtime.txt` to the root of the GitHub repository with a single like that specifies the R version you would like Binder to install. In the [example](https://github.com/davidfastovich/R-Jupyter-Tutorial/blob/main/runtime.txt), I ask Binder to use R version 3.6 released on 2019-04-12 but adding this single line to the text file:

```
r-3.6-2019-04-12
```

Important note - Binder builds R environments from [MRAN](https://mran.microsoft.com/), the Microsoft R Application Network, which is usually a few releases behind [CRAN](https://cran.r-project.org/), The Comprehensive R Archive Network. So be sure to use a version of R that's hosted on MRAN.

After that, if your code has any package dependencies, you specify these to install in a file named `install.R` that is located in the root of the GitHub repository. Using the previous example, the [`install.R`](https://github.com/davidfastovich/R-Jupyter-Tutorial/blob/main/install.R) file has the following lines to install required packages:

```
install.packages("tidyverse")
install.packages("rmarkdown")
install.packages("httr")
install.packages("shinydashboard")
install.packages("leaflet")
install.packages("neotoma")
install.packages("rioja")
install.packages("Bchron")
```

From here, all you need to do is create a live link to Binder in a markdown file on the root of the GitHub repository. The structure of the link follows a simple syntax where:

```
[![Binder](https://mybinder.org/badge_logo.svg)]
```

is markdown for "link this text inside of this badge image", and

```
(https://mybinder.org/v2/gh/davidfastovich/R-Jupyter-Tutorial/main?urlpath=lab)
```

tells Binder what repository to download and what program to open it in. Here, `https://mybinder.org/v2/gh/` links to Binder v2 and specifies that it should look at a GitHub repository (`gh`), from user `davidfastovich`, titled `R-Jupyter-Tutorial`. The last bit, `/main?urlpath=lab` tells Binder to open JupyterLab on launch, but this can be configured to open RStudio by changing it to `main?urlpath=rstudio`. So, the line in the markdown file would read either

```
![Binder](https://mybinder.org/badge_logo.svg)](https://mybinder.org/v2/gh/davidfastovich/R-Jupyter-Tutorial/main?urlpath=lab)
```

for JupyterLab or

```
[![Binder](https://mybinder.org/badge_logo.svg)](https://mybinder.org/v2/gh/davidfastovich/R-Jupyter-Tutorial/main?urlpath=rstudio)
```

for RStudio. 

And that's all there is to it! This is quite a lot of text for a simple process: you create three files on the root of the GitHub repository and link to Binder. In all, this should take ~5 minutes and can greatly extend open science!

Below are two links that will launch the example repository in Binder in JupyterLab or RStudio:

Jupyterlab [![Binder](https://mybinder.org/badge_logo.svg)](https://mybinder.org/v2/gh/davidfastovich/R-Jupyter-Tutorial/main?urlpath=lab)

RStudio [![Binder](https://mybinder.org/badge_logo.svg)](https://mybinder.org/v2/gh/davidfastovich/R-Jupyter-Tutorial/main?urlpath=rstudio)

Hopefully, playing with code within Binder has demonstrated its use for open science, but also demonstrated that Binder can be a powerful educational tool. I've taken many coding classes and the first barrier to entry was getting the coding environment set up on a personal computer. Binder resolves that issue entirely - **students don't have to worry about installing or upgrading anything, it's all handled by the instructor.**