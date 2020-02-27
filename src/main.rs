use crossterm::{
  cursor,
  event::{poll, read, Event, KeyCode, KeyEvent},
  style::{self, Colorize},
  terminal, ExecutableCommand, QueueableCommand, Result,
};
use lib::snake::Snake;
use std::io::{stdout, Stdout, Write};
use std::time::Duration;

fn wrapper<F>(f: F) -> Result<()>
where
  F: FnOnce(&mut Stdout) -> Result<()>,
{
  let mut stdout = stdout();

  //preset the window
  terminal::enable_raw_mode()?;
  stdout
    .execute(terminal::EnterAlternateScreen)?
    .execute(terminal::Clear(terminal::ClearType::All))?
    .execute(cursor::Hide)?;

  f(&mut stdout)?;

  //cleaning up
  terminal::disable_raw_mode()?;
  stdout
    .execute(cursor::Show)?
    .execute(terminal::Clear(terminal::ClearType::All))?
    .execute(terminal::LeaveAlternateScreen)?;

  Ok(())
}

fn matrix(stdout: &mut Stdout) -> Result<()> {
  let (col, row) = terminal::size().unwrap();

  let mut snakes: Vec<Snake> = Vec::with_capacity(col as usize);

  //init buffer
  for _ in 0..(col + 1) {
    snakes.push(Snake::new(10, row, 0));
  }

  let a = ["0", "1"];

  let mut it = a.iter().cycle();

  loop {
    for x in 0..(col + 1) {
      let snake: &mut Snake = &mut snakes[x as usize];

      if snake.visit() {
        for y in snake.current.iter() {
          let content = if y == snake.current.last().unwrap() {
            it.next().unwrap().white()
          } else {
            it.next().unwrap().magenta()
          };

          stdout
            .queue(cursor::MoveTo(x, *y))?
            .queue(style::PrintStyledContent(content))?;
        }
      }
    }

    stdout.flush()?;

    if poll(Duration::from_millis(100))? {
      match read()? {
        Event::Key(KeyEvent { code, modifiers: _ }) => match code {
          KeyCode::Char('q') | KeyCode::Esc => break,
          _ => continue,
        },
        _ => continue,
      }
    }

    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
  }

  Ok(())
}

fn main() -> Result<()> {
  wrapper(matrix)
}
