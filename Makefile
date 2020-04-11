IMAGE=garugaru/garu-io-projects-api
COMMIT=$(shell git rev-parse --short HEAD)


docker-build:
	docker build -f Dockerfile.amd64   -t ${IMAGE}:amd64-latest   -t ${IMAGE}:amd64-${COMMIT}  .
	docker build -f Dockerfile.arm32v7 -t ${IMAGE}:arm32v7-latest -t ${IMAGE}:arm32v7-${COMMIT}  .
	docker push ${IMAGE}:amd64-${COMMIT}
	docker push ${IMAGE}:arm32v7-${COMMIT}

	docker manifest create --amend ${IMAGE}:${COMMIT} ${IMAGE}:amd64-${COMMIT} ${IMAGE}:arm32v7-${COMMIT}
	docker manifest annotate ${IMAGE}:${COMMIT} ${IMAGE}:amd64-${COMMIT}   --os linux --arch amd64
	docker manifest annotate ${IMAGE}:${COMMIT} ${IMAGE}:arm32v7-${COMMIT} --os linux --arch arm --variant v7
	docker manifest push  ${IMAGE}:${COMMIT}

	docker manifest create --amend ${IMAGE}:latest ${IMAGE}:amd64-latest ${IMAGE}:arm32v7-latest
	docker manifest annotate ${IMAGE}:latest ${IMAGE}:amd64-latest   --os linux --arch amd64
	docker manifest annotate ${IMAGE}:latest ${IMAGE}:arm32v7-latest --os linux --arch arm --variant v7
	docker manifest push  ${IMAGE}:latest