test:
    cargo watch -q -c -w src/ -w tests/ -x 'test -- --test-threads=1 --nocapture'