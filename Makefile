lint-docs: # lint docs
	markdownlint '**/*.md' --ignore '**/target/**/*.md'
