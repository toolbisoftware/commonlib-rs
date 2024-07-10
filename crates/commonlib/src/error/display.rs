// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use std::error::Error as _;

use super::Error;

const INDENTATION: &str = "    ";

macro_rules! flatten_errors {
  ($error:expr) => {{
    let mut result = vec![($error.message.clone(), &$error.category, &$error.location)];
    let mut current = $error.source();

    while let Some(error) = current {
      match error.downcast_ref::<Error>() {
        Some(error) => result.push((error.message.clone(), &error.category, &error.location)),
        None => result.push((error.to_string(), &None, &None)),
      };

      current = error.source();
    }

    result
  }};
}

macro_rules! group_errors {
  ($errors:expr) => {{
    let mut groups = Vec::new();

    for (message, category, location) in $errors {
      let mut elements = Vec::new();

      let category = category
        .as_ref()
        .map(|c| format!(" ({})", c))
        .unwrap_or("".into());

      elements.push(format!("error{}: {}", category, message));

      if let Some(location) = location {
        elements.push(format!(
          "at {}:{}:{}",
          location.file, location.line, location.column
        ));
      }

      groups.push(elements);
    }

    groups
  }};
}

fn get_deepness(deepness: usize, is_title: bool) -> Option<usize> {
  match deepness {
    0 => None,
    1 => match is_title {
      true => None,
      false => Some(1),
    },
    _ => match is_title {
      true => Some(deepness - 1),
      false => Some(deepness),
    },
  }
}

fn indent_line(deepness: usize, is_title: bool) -> String {
  let deepness = get_deepness(deepness, is_title);

  match deepness {
    Some(d) => {
      format!("{}", INDENTATION.repeat(d))
    }
    None => "".into(),
  }
}

fn format_lines(groups: Vec<Vec<String>>) -> Vec<String> {
  let mut result = Vec::new();
  let mut deepness = 0;

  for (idx, group) in groups.iter().enumerate() {
    let is_first_group = idx == 0;
    let is_last_group = groups.len() == idx + 1;

    for (idx, element) in group.iter().enumerate() {
      let is_first_element = idx == 0;
      let is_last_element = group.len() == idx + 1;

      if is_first_element {
        match is_first_group {
          true => result.push(element.clone()),
          false => result.push(format!("{}╰─▶ {}", indent_line(deepness, true), element)),
        }

        continue;
      }

      if !is_last_group || !is_last_element {
        let indentation = indent_line(deepness, false);

        result.push(format!("{}├╴{}", indentation, element));

        if is_last_element {
          result.push(format!("{}│", indentation));
        }

        continue;
      }

      result.push(format!("{}╰╴{}", indent_line(deepness, false), element));
    }

    deepness += 1;
  }

  result
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let flattened_errors = flatten_errors!(self);
    let grouped_errors = group_errors!(flattened_errors);

    let lines = format_lines(grouped_errors);
    for line in lines {
      writeln!(f, "{}", line)?;
    }

    Ok(())
  }
}
