.PHONY: dev build docker-build kube-deploy kube-delete kube-logs kube-status

dev:
	@echo "Starting Constellation..."
	@cd backend && cargo watch -x run &
	@cd frontend && npm run dev

build:
	@echo "Building Constellation..."
	@cd backend && cargo build --release

docker-build:
	@echo "Building Docker image..."
	@cd backend && docker build -t docker.io/library/constellation:latest .

kube-deploy: docker-build
	@echo "Deploying to Kubernetes..."
	@kubectl apply -f kubernetes/

kube-delete:
	@echo "Deleting Kubernetes deployment..."
	@kubectl delete -f kubernetes/

kube-logs:
	@echo "Showing logs..."
	@kubectl logs -f deployment/constellation

kube-status:
	@echo "Kubernetes Status:"
	@echo "\nDeployments:"
	@kubectl get deployments
	@echo "\nPods:"
	@kubectl get pods
	@echo "\nServices:"
	@kubectl get services

deploy: build docker-build kube-deploy kube-status

clean: kube-delete
	@echo "Cleaning up..."
	@docker rmi constellation:latest || true
