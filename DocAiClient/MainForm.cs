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

                if (response.IsSuccessStatusCode)
                {
                    try
                    {
                        var doc = JsonDocument.Parse(body);
                        string pretty = JsonSerializer.Serialize(doc, new JsonSerializerOptions { WriteIndented = true });
                        txtResult.Text = $"Category: {selectedCategory}\n\n{pretty}";
                    }
                    catch
                    {
                        txtResult.Text = "Received success but response is not valid JSON:\n\n" + body;
                    }
                }
                else
                {
                    txtResult.Text = $"Server error {response.StatusCode} ({response.ReasonPhrase})\n\n{body}";
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