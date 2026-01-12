# Configuration file for the Sphinx documentation builder.
#
# For the full list of built-in configuration values, see the documentation:
# https://www.sphinx-doc.org/en/master/usage/configuration.html

# -- Project information -----------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#project-information

from sphinx.highlighting import lexers

project = 'Dango'
copyright = '2026, raiseAfloppaFan3925'
author = 'raiseAfloppaFan3925'
release = '0.11.0'

# -- General configuration ---------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#general-configuration

extensions = [
    # 'notfound.extension'
    'sphinx_design',
]

templates_path = ['_templates']
exclude_patterns = ['_build', 'Thumbs.db', '.DS_Store']



# -- Options for HTML output -------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#options-for-html-output

html_theme = 'shibuya'
html_static_path = ['_static']
html_logo = 'assets/images/dango-lang-logo.svg'

from pygments.lexer import RegexLexer
from pygments import token

class DangoLexer(RegexLexer):
    name = 'dango'

    tokens = {
        'root': [
            (r'eat', token.Keyword),
            (r'fetch\s+[0-9]+', token.Keyword),
            (r'skewer\s+[0-9]+', token.Keyword),
            (r'remove', token.Keyword),
            (r'(\-\-\-\-)', token.Punctuation),
            (r'\([\+|\-|\*|/|=|!=|>|<]\)', token.Operator),
            (r'\([\'|\'c|`|;|]\)', token.Literal),
            (r'\(j|while|len\)', token.Operator),
            (r'(\(-?[0-9]+(|\.[0-9]+)\))', token.Number),
            (r'(\(:.*?\))', token.Other),
            (r'\([^:].*?\)', token.Literal),
            (r'\s+', token.Whitespace),
        ]
    }

lexers[DangoLexer.name] = DangoLexer()
pygments_style = 'sphinx'
