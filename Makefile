
all:
	rustc virus.rs
	- rm -f virus.pdb
	- rm -f .gitattributes