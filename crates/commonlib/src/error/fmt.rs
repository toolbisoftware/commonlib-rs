// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

const INDENTATION: &str = "    ";

#[derive(Debug)]
pub struct ErrorFmtGroup {
  pub elements: Vec<String>,
}

pub fn format(groups: Vec<ErrorFmtGroup>) -> Vec<String> {
  let mut lines = Vec::new();
  let mut deepness = 0;

  let group_amount = groups.len();
  for (idx, group) in groups.iter().enumerate() {
    let is_first_group = idx == 0;
    let is_last_group = group_amount == idx + 1;

    let element_amount = group.elements.len();
    for (idx, element) in group.elements.iter().enumerate() {
      let is_first_element = idx == 0;
      let is_last_element = element_amount == idx + 1;

      if is_first_element {
        match is_first_group {
          true => {
            lines.push(element.clone());
          }
          false => lines.push(format!("{}╰─▶ {}", indentation(deepness, true), element)),
        }

        continue;
      }

      if !is_last_group || !is_last_element {
        let indentation = indentation(deepness, false);

        lines.push(format!("{}├╴{}", indentation, element));

        if is_last_element {
          lines.push(format!("{}│", indentation));
        }

        continue;
      }

      lines.push(format!("{}╰╴{}", indentation(deepness, false), element));
    }

    deepness += 1;
  }

  lines
}

fn indentation(deepness: usize, is_title: bool) -> String {
  let converted_deepness = match deepness {
    0 => None,
    1 => match is_title {
      true => None,
      false => Some(1),
    },
    _ => match is_title {
      true => Some(deepness - 1),
      false => Some(deepness),
    },
  };

  match converted_deepness {
    Some(d) => {
      format!("{}", INDENTATION.repeat(d))
    }
    None => "".to_string(),
  }
}
