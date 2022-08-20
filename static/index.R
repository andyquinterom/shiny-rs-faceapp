library(shiny)
library(devtools)
library(bslib)
library(htmltools)

# This function just get's the version of an R package you have installed.
# You could use Renv or anyother means of setting a static version if you need
# it.
get_package_version <- function(pkg) {
  ns <- .getNamespace(pkg)
  if (is.null(ns)) {
    utils::packageVersion(pkg)
  } else {
    as.package_version(ns$.__NAMESPACE__.$spec[["version"]])
  }
}

# The following functions add the HTML dependecies
# needed for `htmltools` to be able to properly
# render our UI. We could package this functions
# into an R package in the future.
jqueryDeps <- htmlDependency(
  "jquery",
  "3.6.0",
  src = "www/shared",
  package = "shiny",
  script = "jquery.min.js",
  all_files = FALSE
)

shinyDependencyCSS <- function(theme) {
  version <- get_package_version("shiny")

  if (!is_bs_theme(theme)) {
    return(htmlDependency(
      name = "shiny-css",
      version = version,
      src = "www/shared",
      package = "shiny",
      stylesheet = "shiny.min.css",
      all_files = FALSE
    ))
  }

  scss_home <- system_file("www/shared/shiny_scss", package = "shiny")
  scss_files <- file.path(scss_home, c("bootstrap.scss", "shiny.scss"))
  scss_files <- lapply(scss_files, sass::sass_file)

  bslib::bs_dependency(
    input = scss_files,
    theme = theme,
    name = "shiny-sass",
    version = version,
    cache_key_extra = version
  )
}

shinyDependencies <- function() {
  list(
    jqueryDeps,
    bslib::bs_dependency_defer(shinyDependencyCSS),
    htmlDependency(
      name = "shiny-javascript",
      version = get_package_version("shiny"),
      src = "www/shared",
      package = "shiny",
      script = "shiny.min.js",
      all_files = FALSE
    )
  )
}

ui <- fluidPage(
  # We must include the dependencies somewhere
  shinyDependencies(),
  title = "Face App",
  fluidRow(
    textInput("path", label = "Image", placeholder = "andres.jpg"),
    div(
      tags$a(tags$b("See the available images"), href = "/img"),
    ),
    actionButton("run_model", "Run model"),
    uiOutput("result")
  )
)

htmltools::save_html(ui, "static/index.html", libdir = "lib")
