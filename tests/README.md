There seams to be no clear standard

The best sources are:
- https://stackoverflow.com/a/9410271/1045684
- https://github.com/windyroad/JUnit-Schema


Verify against Schema

```
xmllint --schema file://$(pwd)/test/JUnit2.xsd junit.xml --noout
```