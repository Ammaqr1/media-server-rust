#!/usr/bin/env python3

from pathlib import Path
import streamlit as st
from dotenv import dotenv_values


BASE_DIR = Path(__file__).resolve().parent
ENV_FILE = BASE_DIR / ".env"


def load_existing_values() -> dict:
    if not ENV_FILE.exists():
        return {}
    return dict(dotenv_values(ENV_FILE))


def save_env_file(values: dict) -> None:
    lines = []
    for key, value in values.items():
        safe_value = str(value).replace("\n", "").strip()
        lines.append(f"{key}={safe_value}")
    ENV_FILE.write_text("\n".join(lines) + "\n", encoding="utf-8")


st.set_page_config(page_title="Token Config Uploader", page_icon="🔐", layout="centered")
st.title("Token Details Uploader")
st.caption("Store API and login details for token generation.")

existing = load_existing_values()

with st.form("token_details_form"):
    phone_no = st.text_input(
        "Phone Number",
        value=existing.get("UPSTOX_PHONE_NUMBER", ""),
        placeholder="e.g. 9876543210",
    )
    api_key = st.text_input(
        "API Key",
        value=existing.get("UPSTOX_API_KEY", ""),
        placeholder="Enter API key",
    )
    api_secret = st.text_input(
        "API Secret",
        value=existing.get("UPSTOX_API_SECRET", ""),
        placeholder="Enter API secret",
        type="password",
    )
    pincode = st.text_input(
        "PIN Code",
        value=existing.get("UPSTOX_PIN_CODE", ""),
        placeholder="Enter 6-digit pin",
        type="password",
    )
    totp_code = st.text_input(
        "TOTP Secret Code",
        value=existing.get("UPSTOX_TOTP_SECRET", ""),
        placeholder="Enter TOTP secret",
        type="password",
    )
    redirect_uri = st.text_input(
        "Redirect URI",
        value=existing.get("UPSTOX_REDIRECT_URI", "https://www.getmydoctor.in/"),
        placeholder="https://your-redirect-url/",
    )

    submitted = st.form_submit_button("Save Details")

if submitted:
    missing = []
    if not phone_no.strip():
        missing.append("Phone Number")
    if not api_key.strip():
        missing.append("API Key")
    if not api_secret.strip():
        missing.append("API Secret")
    if not pincode.strip():
        missing.append("PIN Code")
    if not totp_code.strip():
        missing.append("TOTP Secret Code")
    if not redirect_uri.strip():
        missing.append("Redirect URI")

    if missing:
        st.error(f"Please fill required fields: {', '.join(missing)}")
    else:
        env_values = {
            "UPSTOX_PHONE_NUMBER": phone_no,
            "UPSTOX_API_KEY": api_key,
            "UPSTOX_API_SECRET": api_secret,
            "UPSTOX_PIN_CODE": pincode,
            "UPSTOX_TOTP_SECRET": totp_code,
            "UPSTOX_REDIRECT_URI": redirect_uri,
        }
        save_env_file(env_values)
        st.success(f"Saved successfully to {ENV_FILE}")

st.divider()
st.write("Current config file:", f"`{ENV_FILE}`")
