package svix

import (
	"context"

	"github.com/svix/svix-libs/go/internal/openapi"
)

type (
	ImportOrganizationIn  openapi.ImportOrganizationIn
	ExportOrganizationOut openapi.ExportOrganizationOut
	SettingsIn            openapi.SettingsIn
	SettingsOut           openapi.SettingsOut
)

type Organization struct {
	api *openapi.APIClient
}

func (o *Organization) Import(importOrganizationIn *ImportOrganizationIn) error {
	req := o.api.OrganizationApi.ImportOrganizationConfigurationApiV1OrgImportPost(context.Background())
	req = req.ImportOrganizationIn(openapi.ImportOrganizationIn(*importOrganizationIn))

	res, err := req.Execute()
	return wrapError(err, res)
}

func (o *Organization) Export() (*ExportOrganizationOut, error) {
	req := o.api.OrganizationApi.ExportOrganizationConfigurationApiV1OrgExportPost(context.Background())
	req = req.Body(map[string]interface{}{})
	resp, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := ExportOrganizationOut(resp)
	return &ret, nil
}
