# frozen_string_literal: true

module Svix
    class OrganizationAPI
        def initialize(api_client)
            @api = OrganizationApi.new(api_client)
        end

        def export
            return @api.export_organization_configuration_api_v1_org_export_post({})
        end

        def import(import_organization_in)
            @api.import_organization_configuration_api_v1_org_import_post(import_organization_in)
            nil
        end
    end
end
