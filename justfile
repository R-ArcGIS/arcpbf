default:
  just --list

update:
  git add . && git commit -m "update" && git push

readme:
  R -q -e "rmarkdown::render('README.Rmd')"
