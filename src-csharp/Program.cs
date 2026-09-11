using Grpc.Core;
using Microsoft.AspNetCore.Builder;
using Microsoft.Extensions.DependencyInjection;
using Domain;

var builder = WebApplication.CreateBuilder(args);
builder.Services.AddGrpc();

var app = builder.Build();
app.MapGrpcService<AuthServiceImpl>();

Console.WriteLine("[C# Core]  50051...");
app.Run("http://127.0.0.1:50051");

public class AuthServiceImpl : AuthEngine.AuthEngineBase
{
    public override Task<UserResponse> ValidateUser(UserRequest request, ServerCallContext context)
    {
        Console.WriteLine($"[C# Core] Anforderung fuer User: {request.Username}@{request.Realm}");

        // Hier echte DB- / Directory-Prüfung einbauen
        bool isValid = request.Username.Equals("admin", StringComparison.OrdinalIgnoreCase);

        return Task.FromResult(new UserResponse
        {
            Exists = isValid,
            IsActive = isValid,
            KeyHash = isValid ? "NTLM_HASH_SECRET_EXAMPLE" : ""
        });
    }
}
