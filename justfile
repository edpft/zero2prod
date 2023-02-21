test:
    cargo watch -q -c -w src/ -w tests/ -x 'test -- --test-threads=1 --nocapture'

test-with-logging:
    TEST_LOG=true cargo watch -q -c -w src/ -w tests/ -x 'test -- --test-threads=1' | bunyan

serve:
    cargo watch -q -c -w src/ -x 'run'

remove-db:
    docker stop pg
    docker rm pg

init-db:
    just remove-db 
    /home/ed/repos/zero2prod/scripts/init_db.sh

create-app:
    doctl apps create --spec spec.yaml

id := `doctl apps list | awk 'FNR == 2 {print $1}'`

update-app:
    doctl apps update {{id}} --spec=spec.yaml

delete-app:
    doctl apps delete {{id}}