package com.svix;

import com.svix.exceptions.ApiException;
import com.svix.internal.api.OrganizationApi;
import com.svix.models.ExportOrganizationOut;
import com.svix.models.ImportOrganizationIn;

public final class Organization {
	private final OrganizationApi api;

	Organization() {
		api = new OrganizationApi();
	}

	public ExportOrganizationOut exportOrganization() throws ApiException {
		try {
			return api.exportOrganizationConfigurationApiV1OrgExportPost(new Object());
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public void importOrganization(final ImportOrganizationIn importOrganizationIn) throws ApiException {
		try {
			api.importOrganizationConfigurationApiV1OrgImportPost(importOrganizationIn);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}
}
