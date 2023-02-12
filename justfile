test:
    cargo watch -q -c -w src/ -x 'test -- --test-threads=1 --nocapture'