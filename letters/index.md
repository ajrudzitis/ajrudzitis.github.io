---
title:  "Letter Archive"
---

Archive of Past Letters
==

<table>
    <tr>
        <th>Date</th>
        <th>Title</th>
    </tr>
{% for post in site.categories.letters %}
    <tr>
        <td>
            {{ post.date | date: "%Y %B %d" }}
        </td>
        <td>
            <a href="{{ post.url }}">{{ post.title }}</a>
        </td>
    </tr>
{% endfor %}
</table>




