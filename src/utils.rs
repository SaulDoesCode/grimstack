use comrak::{markdown_to_html, ComrakOptions};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use regex::Regex;
use sled::IVec;
use time::OffsetDateTime;

use std::{convert::TryInto, lazy::SyncLazy};

static COMRAK_OPTS: SyncLazy<ComrakOptions> = SyncLazy::new(|| {
  let mut md_opts = ComrakOptions::default();
  md_opts.parse.smart = true;
  md_opts.render.hardbreaks = true;
  md_opts.render.github_pre_lang = true;
  md_opts.extension.header_ids = Some("writ-".to_string());
  md_opts.extension.autolink = true;
  md_opts.extension.footnotes = true;
  md_opts.extension.table = true;
  md_opts.extension.tasklist = true;
  md_opts.extension.tagfilter = true;
  md_opts.extension.strikethrough = true;
  md_opts.extension.superscript = true;
  md_opts.extension.description_lists = true;

  md_opts
});

static EMAIL_REGEX: SyncLazy<Regex> = SyncLazy::new(|| {
  Regex::new(
    r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})"
  ).unwrap()
});

pub fn unix_timestamp() -> i64 {
  OffsetDateTime::now_utc().unix_timestamp()
}

pub fn datetime_from_unix_timestamp(timestamp: i64) -> OffsetDateTime {
  OffsetDateTime::from_unix_timestamp(timestamp)
}

pub trait FancyBool {
  fn qualify<T>(&self, data: T) -> Option<T>;
  fn wrap(&self) -> Option<bool>;
}

impl FancyBool for bool {
  fn qualify<T>(&self, data: T) -> Option<T> {
    if *self {
      return Some(data);
    }
    None
  }

  #[inline]
  fn wrap(&self) -> Option<bool> {
    if *self {
      return Some(true);
    }
    None
  }
}

pub trait FancyIVec {
  fn to_string(&self) -> String;
  fn to_str(&self) -> &str;
  fn to_u64(&self) -> u64;
  fn to_i64(&self) -> i64;
  fn to_type_from_json<T: serde::de::DeserializeOwned>(&self) -> T;

  fn from_u64(i: u64) -> IVec;
  fn from_i64(i: i64) -> IVec;
}

impl FancyIVec for IVec {
  fn to_string(&self) -> String {
    std::str::from_utf8(self).unwrap().to_owned()
  }

  fn to_str(&self) -> &str {
    std::str::from_utf8(self).unwrap()
  }

  fn to_u64(&self) -> u64 {
    let input = &mut self.as_ref();
    let (int_bytes, rest) = input.split_at(std::mem::size_of::<u64>());
    *input = rest;
    u64::from_be_bytes(int_bytes.try_into().unwrap())
  }

  fn to_i64(&self) -> i64 {
    let input = &mut self.as_ref();
    let (int_bytes, rest) = input.split_at(std::mem::size_of::<i64>());
    *input = rest;
    i64::from_be_bytes(int_bytes.try_into().unwrap())
  }

  fn to_type_from_json<T: serde::de::DeserializeOwned>(&self) -> T {
    // simd_json::from_slice(&mut input).unwrap()
    let input = self.to_vec();
    serde_json::from_slice(&input).unwrap()
  }

  fn from_u64(i: u64) -> IVec {
    IVec::from(&i.to_be_bytes())
  }

  fn from_i64(i: i64) -> IVec {
    IVec::from(&i.to_be_bytes())
  }
}

/*

pub fn generate_random_bytes(len: usize) -> Vec<u8> {
  (0..len).map(|_| rand::random::<u8>()).collect()
}

*/

pub fn random_string(len: usize) -> String {
  thread_rng()
  .sample_iter(&Alphanumeric)
  .map(char::from)
  .take(len).collect()
}
/*
#[inline]
pub fn word_count_bytes(text: &[u8]) -> usize {
  text.split(is_whitespace).count()
}

#[inline]
fn is_whitespace(c: &u8) -> bool {
  *c == b' ' || *c == b'\t' || *c == b'\n'
}

pub fn is_char_number_or_uppercase(c: char) -> bool {
  c.is_numeric() || c.is_uppercase()
}
*/

pub fn is_char_username_unfriendly(c: char) -> bool {
  !c.is_alphanumeric()
}

#[inline]
pub fn is_username_ok(username: &str) -> bool {
  let len = username.len();
  len >= 3 && len <= 50
    && !username.starts_with(' ')
    && !username.ends_with(' ')
    && !username.contains("  ")
    && !username.starts_with('-')
    && !username.ends_with('-')
    && !username.contains("--")
    && !username.contains("--")
    && !username.starts_with('_')
    && !username.ends_with('_')
    && !username.contains("__")
    && username.find(is_char_username_unfriendly).is_none()
}

#[inline]
pub fn is_handle_ok(handle: &str) -> bool {
  let len = handle.len();
  len >= 3 && len <= 50
    && !handle.starts_with(' ')
    && !handle.ends_with(' ')
    && !handle.contains("  ")
    && !handle.starts_with('-')
    && !handle.ends_with('-')
    && !handle.contains("--")
    && !handle.contains("--")
    && !handle.starts_with('_')
    && !handle.ends_with('_')
    && !handle.contains("__")
    && handle.find(is_char_username_unfriendly).is_none()
}

pub fn is_email_ok(email: &str) -> bool {
  EMAIL_REGEX.is_match(email)
}

pub fn i64_is_zero(i: &i64) -> bool {
  *i == 0
}
/*
pub fn read_be_u64(input: &mut &[u8]) -> u64 {
  let (int_bytes, rest) = input.split_at(std::mem::size_of::<u64>());
  *input = rest;
  u64::from_be_bytes(int_bytes.try_into().unwrap())
}

pub fn read_be_u64_from_ivec(ivec: IVec) -> u64 {
  let input = &mut ivec.as_ref();
  let (int_bytes, rest) = input.split_at(std::mem::size_of::<u64>());
  *input = rest;
  u64::from_be_bytes(int_bytes.try_into().unwrap())
}

pub fn read_be_i64(input: &mut &[u8]) -> i64 {
  let (int_bytes, rest) = input.split_at(std::mem::size_of::<i64>());
  *input = rest;
  i64::from_be_bytes(int_bytes.try_into().unwrap())
}

pub fn read_be_i64_from_ivec(ivec: IVec) -> i64 {
  let input = &mut ivec.as_ref();
  let (int_bytes, rest) = input.split_at(std::mem::size_of::<i64>());
  *input = rest;
  i64::from_be_bytes(int_bytes.try_into().unwrap())
}

pub fn string_from_ivec(ivec: IVec) -> String {
  std::str::from_utf8(&ivec).unwrap().to_owned()
}
*/
pub fn render_md(html: &str) -> String {
  markdown_to_html(html, &COMRAK_OPTS)
}

/*
pub fn get_local_ip() -> Option<Ipv4Addr> {
  if cfg!(target_os = "windows") {
    extern crate ipconfig;
    if let Ok(adapters) = ipconfig::get_adapters() {
      for adapter in adapters {
          println!("Ip addresses: {:#?}", adapter.ip_addresses());
          println!("Dns servers: {:#?}", adapter.dns_servers());
      }
    }
  } else if cfg!(target_os = "linux") {
    let output = match Command::new("hostname").args(&["-I"]).output() {
        Ok(ok) => ok,
        Err(_) => return None
    };

    let stdout = match String::from_utf8(output.stdout) {
        Ok(ok) => ok,
        Err(_) => return None
    };

    let ips: Vec<&str> = stdout.trim().split(" ").collect();
    if let Some(first) = ips.first() {
      if !first.is_empty() {
        if let Ok(addr) = first.parse::<Ipv4Addr>() {
            return Some(addr);
        }
        /* else if let Ok(addr) = first.parse::<Ipv6Addr>() {
            return Some(IpAddr::V6(addr));
        }*/
      }
    }
  }
  None
} */
