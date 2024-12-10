terraform {
  required_version = ">1.8.0"
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = ">= 5.0"
    }
  }
  backend "s3" {
    region         = "{{ region_name }}"
    bucket         = "{{ project_name }}-terraform-state"
    dynamodb_table = "{{ project_name }}-terraform-state-lock"
    key            = "terraform-{{ env_name }}.tfstate"
    encrypt        = true
  }
}

variable "region" {
  default = "{{ region_name }}"
  type    = string
}

locals {
  project_name    = "{{ project_name }}"
  env_name        = "{{ env_name }}"
  resource_prefix = "${local.project_name}-${local.env_name}"
}

provider "aws" {
  region = var.region

  default_tags {
    tags = {
      prd_name = "${local.project_name}-${local.env_name}"
    }
  }
}
