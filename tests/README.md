There seams to be no clear standard

The best sources are:
- https://stackoverflow.com/a/9410271/1045684
- https://github.com/windyroad/JUnit-Schema


Verify against Schema

```
xmllint --schema file://$(pwd)/test/JUnit2.xsd junit.xml --noout
```

## Prerequisites

The `xmllint` tool is required to verify XML output against the schema, it is part of the libxml2 package.

### Installation

**Debian/Ubuntu:**
```
sudo apt-get install libxml2-utils
```

**Fedora:**
```
sudo dnf install libxml2
```

**NixOS:**
```
nix-env -iA nixpkgs.libxml2
```

Or temporarily in a shell:
```
nix-shell -p libxml2
```