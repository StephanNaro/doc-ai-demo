// SPDX-License-Identifier: GPL-3.0-or-later

using System;
using System.Net.Http;
using System.Text;
using System.Text.Json;
using System.Threading.Tasks;
using System.Windows.Forms;
using MyHttpClients;

namespace DocAiClient
{
    public partial class MainForm : Form
    {
        private readonly RestHttpClient _client = new RestHttpClient();
        private const string ApiUri = "http://localhost:8001/query";

        public MainForm()
        {
            InitializeComponent();

            // Wire up button click
            btnAsk.Click += async (s, e) => await AskButton_Click();
        }

        private async Task AskButton_Click()
        {
            string query = txtQuery.Text.Trim();
            if (string.IsNullOrEmpty(query))
            {
                MessageBox.Show("Please enter a question.", "Input required", MessageBoxButtons.OK, MessageBoxIcon.Warning);
                return;
            }

            string selectedCategory = cmbCategory.SelectedItem?.ToString() ?? "Invoices";

            // Map display name → API category value
            string apiCategory = selectedCategory switch
            {
                "Employment Contracts" => "contracts",
                "Customer Support"     => "support",
                "Knowledge Base"       => "knowledge",
                _                      => "invoices" // default
            };

            btnAsk.Enabled = false;
            txtResult.Text = "Thinking...";

            try
            {
                var payload = new { query, category = apiCategory };
                string json = JsonSerializer.Serialize(payload);

                var response = await _client.SendRequestAsync(ApiUri, json);
                string body = await RestHttpClient.GetResponseBodyAsync(response);

                // Always expect 200 now
                if (!response.IsSuccessStatusCode)
                {
                    txtResult.Text = $"Unexpected server status: {response.StatusCode} ({response.ReasonPhrase})\r\n\r\nRaw response:\r\n{body}";
                    return;
                }

                // Parse the envelope
                try
                {
                    var doc = JsonDocument.Parse(body);
                    var root = doc.RootElement;

                    if (root.TryGetProperty("success", out var successProp) && successProp.ValueKind == JsonValueKind.True)
                    {
                        // Success → show data
                        if (root.TryGetProperty("data", out var dataProp))
                        {
                            string pretty = JsonSerializer.Serialize(
                                dataProp,
                                new JsonSerializerOptions { WriteIndented = true }
                            );

                            txtResult.Text = $"Category: {selectedCategory}\r\n\r\n{pretty}";
                        }
                        else
                        {
                            txtResult.Text = "Success response missing 'data' field:\r\n\r\n" + body;
                        }
                    }
                    else
                    {
                        // Error path
                        string errorMsg = "Unknown error";

                        if (root.TryGetProperty("error", out var errorProp) && errorProp.ValueKind == JsonValueKind.Object)
                        {
                            var errObj = errorProp;

                            string code = errObj.TryGetProperty("code", out var c) ? c.GetString() ?? "unknown" : "unknown";
                            string message = errObj.TryGetProperty("message", out var m) ? m.GetString() ?? "" : "";
                            string cat = errObj.TryGetProperty("category", out var catProp) ? catProp.GetString() ?? "" : "";
                            string q = errObj.TryGetProperty("query", out var qProp) ? qProp.GetString() ?? "" : "";

                            errorMsg = $"Error ({code}): {message}";
                            if (!string.IsNullOrEmpty(cat)) errorMsg += $"\r\n\r\nCategory: {cat}\r\n";
                            if (!string.IsNullOrEmpty(q)) errorMsg += $"\r\nQuery: \"{q}\"\r\n";
                        }
                        else
                        {
                            errorMsg = "Error response missing 'error' object:\r\n\r\n" + body;
                        }

                        txtResult.Text = errorMsg;
                    }
                }
                catch (JsonException ex)
                {
                    txtResult.Text = $"Invalid JSON response:\r\n{ex.Message}\r\n\r\nRaw body:\r\n{body}";
                }
            }
            catch (Exception ex)
            {
                txtResult.Text = $"Client error: {ex.Message}";
            }
            finally
            {
                btnAsk.Enabled = true;
            }
        }
    }
}