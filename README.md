<!-- README.md -->

<div align="center">
  <h1 id="top" align="center">Football Analytics</h1>
<h3 align="center">A space for football analytics projects by me, including a curated list of publicly available resources </h3>
<p align="center">
  
![GitHub Last Commit](https://img.shields.io/github/last-commit/jamesdidathing/football_analysis?style=plastic)
![GitHub Commit Activity](https://img.shields.io/github/commit-activity/m/jamesdidathing/football_analysis.svg)
![GitHub Repository Size](https://img.shields.io/github/repo-size/jamesdidathing/football_analysis?style=plastic)
</p>

</div>

This repo is inspired and gives credit to [`Edd Wesbter`]([https://numpy.org/doc/stable/contents.html](https://github.com/eddwebster/football_analytics/tree/master)) who curated much of this info.
<!-- TABLE OF CONTENTS -->
<h2 id="table-of-contents"> 📍 Table of Contents</h2>

<details open="open">
  <summary>Table of Contents</summary>
  <ol>
    <li><a href="#getting-started"> 🚀 Getting Started</a></li>
    <li><a href="#repository-structure"> 🌵 Repository Structure</a></li>
    <li><a href="#tutorials">🧑‍🎓 Tutorials</a></li>
    <li><a href="#analysis">📍 Current Analysis</a></li>
  </ol>

<!-- GETTING STARTED -->
<h2 id="getting-started"> 🚀 Getting Started</h2>


<h3 id="getting-started-dependencies"> ✅ Dependencies</h3>

The code in this repository is written in Python. Before you begin, ensure that you have the following prerequisites installed:
1. Python (ideally 3.6.1+ installed)
3. The following Python libraries...

<h4 id="getting-started-dependencies-python"> 🐍 Python</h3>

General Python data science libraries:
*    [`NumPy`](https://numpy.org/doc/stable/contents.html) for multidimensional array computing;
*    [`pandas`](http://pandas.pydata.org/) for data analysis and manipulation;
*    [`matplotlib`](https://matplotlib.org/contents.html?v=20200411155018) and [`Seaborn`](https://seaborn.pydata.org/) for data visualisation; and
*    [`scitkit-learn`](https://scikit-learn.org/stable/index.html) and [`SciPy`](https://www.scipy.org/) for Machine Learning.

Football analytics Python libraries:
*    [`matplotsoccer`](https://github.com/TomDecroos/matplotsoccer) - a Python library for visualising soccer event data by [Tom Decroos](https://twitter.com/TomDecroos)
*    [`mplsoccer`](https://github.com/andrewRowlinson/mplsoccer) - a Python library for plotting football pitches in matplotlib by [Andrew Rowlinson](https://twitter.com/numberstorm)
*    [`statsbombapi`](https://github.com/Torvaney/statsbombapi) - a Python API wrapper and dataclasses for [StatsBomb](https://statsbomb.com/) data
*    [`statsbombpy`](https://github.com/statsbomb/statsbombpy) - a Python library written by Francisco Goitia to access [StatsBomb](https://statsbomb.com/) data
*    [`socceraction`](https://github.com/ML-KULeuven/socceraction) - a Python library for valuing the individual actions performed by soccer players. Includes an Expected Threat (xT) 


<p align="right">
  <a href="#top"><b>🔝 Return </b></a>
</p>

<!-- REPOSITORY STRUCTURE -->
<h2 id="repository-structure"> 🌵 Repository Structure</h2>

The contents of this GitHub repository is organised as follows:

    📂 jamesdidathing/football_analysis/ ➡️ central repository of code and analysis 📍⚽
    │
    ├── 📂 soccermatics/ ➡️ jupyter notebooks and data from which I am learning various aspects of football analysis
    │   ├── 📂 data
    │   ├── 📂 venv
    │   ├── plotting_shots.ipynb
    │   ├── sign_test.ipynb
    │   ├── using_wyscout.ipynb
    │   └── using_statsbomb.ipynb   

<p align="right">
  <a href="#top"><b>🔝 Return </b></a>
</p>


<h3 id="tutorials"> :student: Tutorials</h3>

<h4 id="tutorials-python"> Python</h4>

*    [Soccermatics](https://soccermatics.readthedocs.io/en/latest/) course taught by David Sumpter, a comprehensive education on how to work with football data.

<p align="right">
  <a href="#top"><b>🔝 Return </b></a>
</p>

<h2 id="analysis"> 📍 Analysis</h2>


### More in depth look into the style of play and individual players across the 2015/16 season wherein Leicester City won the Premier League title.

* The first infographic displays areas of the pitch where teams most often created a dangerous pass (a pass that led to a shot within 15s of the pass) through a heatmap. We can see through this that some teams (Newcastle, Manchester Utd, Chelsea, Swansea) typically created these chances through one side of the pitch. The title winners this season, Leicester, created a low number of dangerous passes per game whereas runners-up Arsenal created much more around the edge of the box.

<p align="center">
  <img src="/images/dangerpass2015:16.png" width="500">
</p>

* The second infographic displays heatmaps for each teams biggest chance creators. Here, chances are defined as a direct pass leading to a shot. One of the more standout players here is Danny Drinkwater, who alongside Vardy, Mahrez and Kante was an often forgotten about key player in Leicester's team. With an average Chance Creation position being much deeper than most other teams, it shows how successful Leicester City's counter attacking was. 

<p align="center">
  <img src="/images/chnces2015:16.png" width="500">
</p>

* The third infographic displays all positions of passes, assists and passes-leading-to-shots of the top 5 assist makers this season. The 2015/16 season was one of Mesut Ozil's best for Arsenal, shown by the vast number of passes made by the number 10 all across the pitch. One reason for Arsenal not being able to catchup with Leicester this season was due to their lack of clinical finishing. This is exemplified by the fact Mesut Ozil had 103 passes leading to shots, over double of Riyad Mahrez's 45 who only had 7 less assists. 

<p align="center">
  <img src="/images/assists2015:16.png" width="500">
</p>

### Areas to improve
For the above, these are the areas that could improve the analysis:
- Dangerous Passes lead to showing teams with a more possession based approach as "better", where Leicester's counter attacking style is what won them so many games.
- Quality of chances/shots needs to be considered with the final infographic, as although Ozil made many more passes-to-shots than Mahrez, it might be a combination of quality of passes and finishing that led to so many missed chances for Arsenal.
- More defensive approached analysis would probably lend Leicester looking 'better' in these graphs, as they had a formidable pairing of Huth and Morgan.

## 2023-2024 Prem Team Analysis

* This series of analysis used web scraping techniques from FBREF to gather team and player data for plotting. The first and second plots are showcasing how teams perform in their progressive passing metrics as well as their shooting and defending metrics. The third plot showcases the players xG against the number of shots taken, with variables in plot point size and colour based on goals scored and number of shots on target.

<p align="center">
  <img src="/images/prem2023-sot.png" width="700">
</p>

<p align="center">
  <img src="/images/prem2023-passes.png" width="700">
</p>

<p align="center">
  <img src="/images/prem2023-xg.png" width="700">
</p>

## 2023-24 Championship Goal Scorers & Assisters Radar Plots

* Again, using web scraping techniques and data from FBREF I have gathered data from the top goal scorers and top assisters for the Championship. To show a comparison in how these players perform, I have created a radar plot to showcase the differences.

<p align="center">
  <img src="/images/champ_top_scorers.png" width="600">
</p>

<p align="center">
  <img src="/images/champ_top_assisters.png" width="600">
</p>

## Other analysis

*    Analysis of the passing network for the England v Sweden WWC match, showcasing the passing pairs and network for Englands approach to the game.

<p align="center">
  <img src="/images/engswedpass.png" width="400">
</p>

1. Visualisation of Thierry Henry's shots across the Invincible 2003/2004 season, including missed shots.
2. Visualisation of all goals scored by Thierry Henry in the 2003/2004 season, where size of the circle represents a scaled xG value. 

<p align="center">
  <img src="/images/henry_shots.png" width="300">
  <img src="/images/henry_xg_map.png" width="300">
</p>
