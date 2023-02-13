test:
    cargo watch -q -c -w src/ -w tests/ -x 'test -- --test-threads=1 --nocapture'

test-with-logging:
    TEST_LOG=true cargo watch -q -c -w src/ -w tests/ -x 'test -- --test-threads=1' | bunyan

serve:
    cargo watch -q -c -w src/ -x 'run'