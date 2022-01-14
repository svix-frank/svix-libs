/*
 * Svix API
 *
 * Welcome to the Svix API documentation!  Useful links: [Homepage](https://www.svix.com) | [Support email](mailto:support+docs@svix.com) | [Blog](https://www.svix.com/blog/) | [Slack Community](https://www.svix.com/slack/)  # Introduction  This is the reference documentation and schemas for the [Svix webhook service](https://www.svix.com) API. For tutorials and other documentation please refer to [the documentation](https://docs.svix.com).  ## Main concepts  In Svix you have four important entities you will be interacting with:  - `messages`: these are the webhooks being sent. They can have contents and a few other properties. - `application`: this is where `messages` are sent to. Usually you want to create one application for each user on your platform. - `endpoint`: endpoints are the URLs messages will be sent to. Each application can have multiple `endpoints` and each message sent to that application will be sent to all of them (unless they are not subscribed to the sent event type). - `event-type`: event types are identifiers denoting the type of the message being sent. Event types are primarily used to decide which events are sent to which endpoint.   ## Authentication  Get your authentication token (`AUTH_TOKEN`) from the [Svix dashboard](https://dashboard.svix.com) and use it as part of the `Authorization` header as such: `Authorization: Bearer ${AUTH_TOKEN}`.  <SecurityDefinitions />   ## Code samples  The code samples assume you already have the respective libraries installed and you know how to use them. For the latest information on how to do that, please refer to [the documentation](https://docs.svix.com/).   ## Idempotency  Svix supports [idempotency](https://en.wikipedia.org/wiki/Idempotence) for safely retrying requests without accidentally performing the same operation twice. This is useful when an API call is disrupted in transit and you do not receive a response.  To perform an idempotent request, pass the idempotency key in the `Idempotency-Key` header to the request. The idempotency key should be a unique value generated by the client. You can create the key in however way you like, though we suggest using UUID v4, or any other string with enough entropy to avoid collisions.  Svix's idempotency works by saving the resulting status code and body of the first request made for any given idempotency key for any successful request. Subsequent requests with the same key return the same result.  Please note that idempotency is only supported for `POST` requests.   ## Cross-Origin Resource Sharing  This API features Cross-Origin Resource Sharing (CORS) implemented in compliance with [W3C spec](https://www.w3.org/TR/cors/). And that allows cross-domain communication from the browser. All responses have a wildcard same-origin which makes them completely public and accessible to everyone, including any code on any site. 
 *
 * API version: 1.4
 */

// Code generated by OpenAPI Generator (https://openapi-generator.tech); DO NOT EDIT.

package openapi

import (
	"encoding/json"
	"time"
)

// ExportOrganizationOut struct for ExportOrganizationOut
type ExportOrganizationOut struct {
	CreatedAt time.Time `json:"createdAt"`
	EventTypes []EventTypeOut `json:"eventTypes"`
	Settings *SettingsOut `json:"settings,omitempty"`
	Version *int32 `json:"version,omitempty"`
}

// NewExportOrganizationOut instantiates a new ExportOrganizationOut object
// This constructor will assign default values to properties that have it defined,
// and makes sure properties required by API are set, but the set of arguments
// will change when the set of required properties is changed
func NewExportOrganizationOut(createdAt time.Time, eventTypes []EventTypeOut) *ExportOrganizationOut {
	this := ExportOrganizationOut{}
	this.CreatedAt = createdAt
	this.EventTypes = eventTypes
	var version int32 = 1
	this.Version = &version
	return &this
}

// NewExportOrganizationOutWithDefaults instantiates a new ExportOrganizationOut object
// This constructor will only assign default values to properties that have it defined,
// but it doesn't guarantee that properties required by API are set
func NewExportOrganizationOutWithDefaults() *ExportOrganizationOut {
	this := ExportOrganizationOut{}
	var version int32 = 1
	this.Version = &version
	return &this
}

// GetCreatedAt returns the CreatedAt field value
func (o *ExportOrganizationOut) GetCreatedAt() time.Time {
	if o == nil {
		var ret time.Time
		return ret
	}

	return o.CreatedAt
}

// GetCreatedAtOk returns a tuple with the CreatedAt field value
// and a boolean to check if the value has been set.
func (o *ExportOrganizationOut) GetCreatedAtOk() (*time.Time, bool) {
	if o == nil  {
		return nil, false
	}
	return &o.CreatedAt, true
}

// SetCreatedAt sets field value
func (o *ExportOrganizationOut) SetCreatedAt(v time.Time) {
	o.CreatedAt = v
}

// GetEventTypes returns the EventTypes field value
func (o *ExportOrganizationOut) GetEventTypes() []EventTypeOut {
	if o == nil {
		var ret []EventTypeOut
		return ret
	}

	return o.EventTypes
}

// GetEventTypesOk returns a tuple with the EventTypes field value
// and a boolean to check if the value has been set.
func (o *ExportOrganizationOut) GetEventTypesOk() (*[]EventTypeOut, bool) {
	if o == nil  {
		return nil, false
	}
	return &o.EventTypes, true
}

// SetEventTypes sets field value
func (o *ExportOrganizationOut) SetEventTypes(v []EventTypeOut) {
	o.EventTypes = v
}

// GetSettings returns the Settings field value if set, zero value otherwise.
func (o *ExportOrganizationOut) GetSettings() SettingsOut {
	if o == nil || o.Settings == nil {
		var ret SettingsOut
		return ret
	}
	return *o.Settings
}

// GetSettingsOk returns a tuple with the Settings field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *ExportOrganizationOut) GetSettingsOk() (*SettingsOut, bool) {
	if o == nil || o.Settings == nil {
		return nil, false
	}
	return o.Settings, true
}

// HasSettings returns a boolean if a field has been set.
func (o *ExportOrganizationOut) HasSettings() bool {
	if o != nil && o.Settings != nil {
		return true
	}

	return false
}

// SetSettings gets a reference to the given SettingsOut and assigns it to the Settings field.
func (o *ExportOrganizationOut) SetSettings(v SettingsOut) {
	o.Settings = &v
}

// GetVersion returns the Version field value if set, zero value otherwise.
func (o *ExportOrganizationOut) GetVersion() int32 {
	if o == nil || o.Version == nil {
		var ret int32
		return ret
	}
	return *o.Version
}

// GetVersionOk returns a tuple with the Version field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *ExportOrganizationOut) GetVersionOk() (*int32, bool) {
	if o == nil || o.Version == nil {
		return nil, false
	}
	return o.Version, true
}

// HasVersion returns a boolean if a field has been set.
func (o *ExportOrganizationOut) HasVersion() bool {
	if o != nil && o.Version != nil {
		return true
	}

	return false
}

// SetVersion gets a reference to the given int32 and assigns it to the Version field.
func (o *ExportOrganizationOut) SetVersion(v int32) {
	o.Version = &v
}

func (o ExportOrganizationOut) MarshalJSON() ([]byte, error) {
	toSerialize := map[string]interface{}{}
	if true {
		toSerialize["createdAt"] = o.CreatedAt
	}
	if true {
		toSerialize["eventTypes"] = o.EventTypes
	}
	if o.Settings != nil {
		toSerialize["settings"] = o.Settings
	}
	if o.Version != nil {
		toSerialize["version"] = o.Version
	}
	return json.Marshal(toSerialize)
}

type NullableExportOrganizationOut struct {
	value *ExportOrganizationOut
	isSet bool
}

func (v NullableExportOrganizationOut) Get() *ExportOrganizationOut {
	return v.value
}

func (v *NullableExportOrganizationOut) Set(val *ExportOrganizationOut) {
	v.value = val
	v.isSet = true
}

func (v NullableExportOrganizationOut) IsSet() bool {
	return v.isSet
}

func (v *NullableExportOrganizationOut) Unset() {
	v.value = nil
	v.isSet = false
}

func NewNullableExportOrganizationOut(val *ExportOrganizationOut) *NullableExportOrganizationOut {
	return &NullableExportOrganizationOut{value: val, isSet: true}
}

func (v NullableExportOrganizationOut) MarshalJSON() ([]byte, error) {
	return json.Marshal(v.value)
}

func (v *NullableExportOrganizationOut) UnmarshalJSON(src []byte) error {
	v.isSet = true
	return json.Unmarshal(src, &v.value)
}


