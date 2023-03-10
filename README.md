# scribe

A general purpose library for managing a directory of plain text markdown notes.
Meant to be used as a shared utility and indexing crate for interacting with markdown repositories, and serve as a backbone for Obsidian-esque open source projects.

### V1: Basic Functionality

The initial functionality will be based around three primary areas: parsing, templating and indexing

##### Parsing

A parsing engine is available to identify a variety of yaml front matter (DATE, TITLE, TAGS), along with Obsidian style internal `[[wiki-links]]`, urls and internal file references.

##### Templating

A parameterized templating library with a few bare bones built-in templates, along with the ability to infer a user defined 'templates/' folder.

##### Indexing

A indexing engine has been created, generating a manifest of all notes provided within a directory of notes. This index provided quick access information for all parsed information, including backlinks. This enables functionality surrounding file transfer, in which upon a file being transfered, all backlinks are updated to accomodate for the new path.

### V2: Extended Functionality

Building upon V1, V2 will focus on adding NLP functionality to the indexing engine, including Named-Entity-Recognition and Search. Allowing for extended user abilities surrounding searching freeform, or navigating by Person of Interest, Location or Organization.
