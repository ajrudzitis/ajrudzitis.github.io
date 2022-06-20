---
layout: default
title: Bookshelf
---

Bookshelf
===

These are a bunch of books on my real, virtual, or mental bookshelf. These are all books that I've read, or have in my queue to read shortly. 

The list is created partially from memory, so it is certainly not exhaustive. However, it represents _most_ of my adult liteary life. 

Just like my real bookshelf, there is no particular order to this list.

{% for book in site.data.bookshelf %}
* {{book.title}} by {{book.author}}
{% endfor %}