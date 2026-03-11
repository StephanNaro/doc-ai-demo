// SPDX-License-Identifier: GPL-3.0-or-later

using System.Net.Http;
using System.Text;
using System.Threading.Tasks;

namespace MyHttpClients
{
    public class RestHttpClient
    {
        public async Task<HttpResponseMessage> SendRequestAsync(string uri, string jsonPayload)
        {
            using var httpClient = new HttpClient();

            var content = new StringContent(jsonPayload, Encoding.UTF8, "application/json");

            try
            {
                return await httpClient.PostAsync(uri, content);
            }
            catch (Exception ex)
            {
                var errorResponse = new HttpResponseMessage(System.Net.HttpStatusCode.InternalServerError)
                {
                    ReasonPhrase = $"HttpClient exception: {ex.Message}"
                };
                return errorResponse;
            }
        }

        public static async Task<string> GetResponseBodyAsync(HttpResponseMessage response)
        {
            if (response.Content == null) return string.Empty;
            return await response.Content.ReadAsStringAsync();
        }
    }
}