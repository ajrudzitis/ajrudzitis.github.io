---
layout: bookshelf.html
title: Bookshelf
---

Bookshelf
===

These are a bunch of books on my real, virtual, or mental bookshelf. These are all books that I've read, or have in my queue to read shortly. 

The list is created partially from memory, so it is certainly not exhaustive. However, it represents _most_ of my adult liteary life. 

Just like my real bookshelf, there is no particular order to this list.

{% if data["librarything-bookshelf"] %}
{% for book_id, book in data["librarything-bookshelf"] %}
* [{{book.title}}](https://www.librarything.com/work/{{book.workcode}}/book/{{book.books_id}}){% if book.primaryauthor and book.primaryauthor != "" %} by {{book.primaryauthor}}{% endif %}
{% endfor %}
{% endif %}
{% if data.misc_books %}
{% for book in data.misc_books %}
{% if book.url %}
* [{{book.title}}]({{book.url}}){% if book.author and book.author != ""  %} by {{book.author}}{% endif %}
{% else %}
* {{book.title}}{% if book.author and book.author != "" %} by {{book.author}}{% endif %}
{% endif %}
{% endfor %}
{% endif %}

[I manage this list using LibraryThing](https://www.librarything.com/home).