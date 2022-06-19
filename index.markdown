---
layout: default
title: Home
---

# About Me

Hi! My name is Aleks Rudzitis. This is my minimalist website to go with
the domain name that I primarily have for personal email. 

# Articles

{% for post in site.categories.articles %}
*  {{ post.date | date: "%Y %B %d" }}: [{{ post.title }}]({{ post.url }})
{% endfor %}

# Letters

I'm trying out a new thing where rather than posting to Facebook, I send out
periodic letters to my family and friends who are interested in what is going
on with me and my family.

[Sign up here.](https://tinyletter.com/aleksrudzitis/)

[Archive of Past Letters](letters/index.html)

# [Haikus](haikus.html)

# Travel

My wife and I maintain an (inactive for now) travel blog at 
[SeeYouInTwoWeeks.com](https://www.seeyouintwoweeks.com)

# Mindfulness and Meditation

I've assembled a collection of inspirational/motivational quotes 
related to meditation, mindfulness, and Buddhism 
<a href="mindfulness-quotes.html">here</a>.

# Contact/Links

* [LinkedIn](https://www.linkedin.com/in/aleksrudzitis/)
* [GitHub](https://github.com/ajrudzitis)
* [Medium](https://medium.com/@aleks.rudzitis)

# Other

* [Emacs Reference Mug Project](refmug/index.html)
