import unittest
import json
import llm_providers_list

class TestLLMProviders(unittest.TestCase):
    def test_list_providers(self):
        """Test listing all provider IDs"""
        providers = llm_providers_list.list_providers()
        self.assertIsInstance(providers, list)
        self.assertIn("openai", providers)
        self.assertIn("anthropic", providers)
        # Check sorted order
        self.assertEqual(providers, sorted(providers))

    def test_list_models(self):
        """Test listing models for a specific provider"""
        models = llm_providers_list.list_models("openai")
        self.assertIsInstance(models, list)
        self.assertIn("gpt-4o", models)
        
        # Test non-existing provider
        with self.assertRaises(ValueError):
            llm_providers_list.list_models("non_existent_provider")

    def test_get_model(self):
        """Test getting detailed model object"""
        model = llm_providers_list.get_model("openai", "gpt-4o")
        self.assertEqual(model.id, "gpt-4o")
        self.assertEqual(model.name, "GPT-4o")
        self.assertTrue(model.supports_tools)
        
        # Test non-existing model
        with self.assertRaises(ValueError):
            llm_providers_list.get_model("openai", "non_existent_model")

    def test_get_provider(self):
        """Test getting detailed provider object"""
        # Test existing provider
        openai = llm_providers_list.get_provider("openai")
        self.assertEqual(openai.label, "OpenAI")
        self.assertTrue(openai.base_url.startswith("https://api.openai.com"))
        self.assertIsInstance(openai.models, list)
        self.assertTrue(len(openai.models) > 0)
        
        # Test model attributes
        model = openai.models[0]
        self.assertTrue(hasattr(model, "id"))
        self.assertTrue(hasattr(model, "name"))
        self.assertTrue(hasattr(model, "description"))
        self.assertTrue(hasattr(model, "supports_tools"))
        self.assertTrue(hasattr(model, "context_length"))
        self.assertTrue(hasattr(model, "input_price"))
        self.assertTrue(hasattr(model, "output_price"))

        # Test non-existing provider
        with self.assertRaises(ValueError):
            llm_providers_list.get_provider("non_existent_provider")

    def test_get_all_providers(self):
        """Test getting all providers as a dictionary"""
        all_providers = llm_providers_list.get_all_providers()
        self.assertIsInstance(all_providers, dict)
        self.assertIn("openai", all_providers)
        self.assertIn("anthropic", all_providers)
        
        # Check content type
        openai = all_providers["openai"]
        self.assertEqual(openai.label, "OpenAI")

    def test_get_provider_info(self):
        """Test getting provider info as JSON string"""
        # Test existing provider
        info_json = llm_providers_list.get_provider_info("openai")
        self.assertIsInstance(info_json, str)
        
        info = json.loads(info_json)
        self.assertEqual(info["label"], "OpenAI")
        self.assertIn("models", info)
        
        # Test non-existing provider
        with self.assertRaises(ValueError):
            llm_providers_list.get_provider_info("non_existent_provider")

    def test_get_all_info(self):
        """Test getting all info as JSON string"""
        all_info_json = llm_providers_list.get_all_info()
        self.assertIsInstance(all_info_json, str)
        
        all_info = json.loads(all_info_json)
        self.assertIsInstance(all_info, dict)
        self.assertIn("openai", all_info)
        self.assertEqual(all_info["openai"]["label"], "OpenAI")

if __name__ == "__main__":
    unittest.main()
