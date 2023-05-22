---
layout: default
title: Bookshelf
---

Bookshelf
===

These are a bunch of books on my real, virtual, or mental bookshelf. These are all books that I've read, or have in my queue to read shortly. 

The list is created partially from memory, so it is certainly not exhaustive. However, it represents _most_ of my adult liteary life. 

Just like my real bookshelf, there is no particular order to this list.

{% for book in site.data.librarything-bookshelf %}
* [{{book[1].title}}](https://www.librarything.com/work/{{book[1].workcode}}/book/{{book[1].books_id}}){% if book[1].primaryauthor and book[1].primaryauthor != "" %} by {{book[1].primaryauthor}}{% endif %}
{% endfor %}
{% for book in site.data.misc_books %}
{% if book.url %}
* [{{book.title}}]({{book.url}}){% if book.author and book.author != ""  %} by {{book.author}}{% endif %}
{% else %}
* {{book.title}}{% if book.author and book.author != "" %} by {{book.author}}{% endif %}
{% endif %}
{% endfor %}

[I manage this list using LibraryThing.](https://www.librarything.com/home). 