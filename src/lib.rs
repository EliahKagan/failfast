pub fn failfast() -> ! {
    std::process::abort();
}

#[cfg(test)]
mod test {
    #[test]
    fn failfast() {
        super::failfast();
    }
}
