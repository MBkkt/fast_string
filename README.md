# FastString

This library implements the standard String API, with such optimizations as:

* SSO (no allocation if string less than 24 characters long)
* COW (clone method works in O (1)).
