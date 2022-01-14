package com.svix.kotlin

import com.svix.kotlin.exceptions.ApiException
import com.svix.kotlin.internal.apis.OrganizationApi
import com.svix.kotlin.models.ExportOrganizationOut
import com.svix.kotlin.models.ImportOrganizationIn

class Organization internal constructor(token: String, options: SvixOptions) {
    private val api = OrganizationApi(options.serverUrl)

    init {
        api.accessToken = token
        api.userAgent = options.getUA()
    }

    suspend fun export(): ExportOrganizationOut {
        try {
            return api.exportOrganizationConfigurationApiV1OrgExportPost(Object())
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun import(importOrganizationIn: ImportOrganizationIn) {
        try {
            api.importOrganizationConfigurationApiV1OrgImportPost(importOrganizationIn)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }
}
