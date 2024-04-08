Current URL: {{ current_url }}
Vertical scroll position: {{ scroll_position }}%

---

History (from oldest to newest):

{% for url in history -%}
- {{ url }}
{% endfor %}

---

{{ elements }}
