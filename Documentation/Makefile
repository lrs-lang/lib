adocs := $(shell find adoc/ -name "*.adoc")
htmls := $(patsubst adoc/%.adoc, html/%.html, $(adocs))

.PHONY: all clean

all: $(htmls)

html/%.html: adoc/%.adoc
	@mkdir -p "$(@D)"
	asciidoctor -a stylesheet! "$<" -o "$@"

clean:
	rm html/*
