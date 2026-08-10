default:
  just --list

update:
  git add . && git commit -m "update" && git push

readme:
  quarto render README.rmd --to gfm
