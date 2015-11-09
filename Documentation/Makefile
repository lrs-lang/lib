adocs := $(shell find adoc/ -name "*.adoc")
htmls := $(patsubst adoc/%.adoc, html/%.html, $(adocs))

.PHONY: all clean

all: $(htmls)

html:
	mkdir html

html/%.html: adoc/%.adoc | html
	asciidoctor "$<" -o "$@"

clean:
	rm -r html
