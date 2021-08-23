# Model report for file:///tmp/top-repos-quality-repos-83vm6_e2/muxi-cs103.git HEAD c88ca66dfffa1a754e6e897666d4967e8feb1519

### Dump

```json
{'created_at': '2021-08-20 23:08:10',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.4.0-81-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '15.8 kB',
 'tags': [],
 'uuid': 'e2f3fb33-2e97-4e1a-9f21-23fe84500d27',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-83vm6_e2/muxi-cs103.git c88ca66dfffa1a754e6e897666d4967e8feb1519

# javascript
28 rules, avg.len. 7.2
## train
PPCR: 0.986165
### report
macro
{'f1-score': 0.9598926430205492,
 'precision': 0.9843160566162616,
 'recall': 0.9394936109780793,
 'support': 42128}
micro
{'f1-score': 0.9816036840106342,
 'precision': 0.9816036840106342,
 'recall': 0.9816036840106342,
 'support': 42128}
weighted
{'f1-score': 0.9814224065122595,
 'precision': 0.9818412033047906,
 'recall': 0.9816036840106342,
 'support': 42128}
### report_full
macro
{'f1-score': 0.9359578116323105,
 'precision': 0.9843160566162616,
 'recall': 0.9002554907311785,
 'support': 42719}
micro
{'f1-score': 0.9747663441253078,
 'precision': 0.9816036840106342,
 'recall': 0.9680235960579602,
 'support': 42719}
weighted
{'f1-score': 0.9740156201864933,
 'precision': 0.9817811942687753,
 'recall': 0.9680235960579602,
 'support': 42719}
## test
PPCR: 0.979747
### report
macro
{'f1-score': 0.9257784430323651,
 'precision': 0.946692671344273,
 'recall': 0.9132208857182597,
 'support': 8127}
micro
{'f1-score': 0.9408145687215455,
 'precision': 0.9408145687215455,
 'recall': 0.9408145687215455,
 'support': 8127}
weighted
{'f1-score': 0.9409124852252924,
 'precision': 0.9444286813309144,
 'recall': 0.9408145687215455,
 'support': 8127}
### report_full
macro
{'f1-score': 0.9012854401179506,
 'precision': 0.946692671344273,
 'recall': 0.8730825662012758,
 'support': 8295}
micro
{'f1-score': 0.9311898672512484,
 'precision': 0.9408145687215455,
 'recall': 0.9217600964436408,
 'support': 8295}
weighted
{'f1-score': 0.9304239725682528,
 'precision': 0.9445663791307127,
 'recall': 0.9217600964436408,
 'support': 8295}
```

## javascript
### Summary
23 rules, avg.len. 6.8

| | |
|-|-|
|Min support|107|
|Max support|9618|
|Min confidence|0.94775390625|
|Max confidence|0.9996899962425232|

### Configuration

```json
{'feature_extractor': {'cutoff_label_support': 80,
                       'debug_parsing': False,
                       'label_composites': '<cut>',
                       'left_features': ['length',
                                         'diff_offset',
                                         'diff_col',
                                         'diff_line',
                                         'internal_type',
                                         'label',
                                         'reserved',
                                         'roles'],
                       'left_siblings_window': 5,
                       'no_labels_on_right': True,
                       'node_features': ['start_line', 'start_col'],
                       'parent_features': ['internal_type', 'roles'],
                       'parents_depth': 2,
                       'return_sibling_indices': False,
                       'right_features': ['length', 'internal_type', 'reserved', 'roles'],
                       'right_siblings_window': 5,
                       'select_features_number': 500,
                       'selected_features': '<cut>'},
 'line_length_limit': 500,
 'lines_ratio_train_trigger': 0.2,
 'lower_bound_instances': 500,
 'optimizer': {'base_model_name_categories': ['sklearn.ensemble.RandomForestClassifier',
                                              'sklearn.tree.DecisionTreeClassifier'],
               'cv': 3,
               'max_depth_categories': [None, 5, 10],
               'max_features_categories': [None, 'auto'],
               'min_samples_leaf_max': 120,
               'min_samples_leaf_min': 90,
               'min_samples_split_max': 240,
               'min_samples_split_min': 180,
               'n_iter': 50,
               'n_jobs': -1},
 'overall_size_limit': 5242880,
 'random_state': 42,
 'test_dataset_ratio': 0.2,
 'trainable_rules': {'attribute_similarity_threshold': 0.98,
                     'base_model_name': 'sklearn.tree.DecisionTreeClassifier',
                     'confidence_threshold': 0.8,
                     'min_samples_leaf': 90,
                     'min_samples_split': 190,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.999. Support: 9618.` |
| 2 | `  -1.reserved = (<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 421.` |
| 3 | `  -1.reserved = {<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.987. Support: 116.` |
| 4 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.995. Support: 107.` |
| 5 | `  •••start_col ≥ 9<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {LITERAL}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 1857.` |
| 6 | `  •••start_col ≥ 9<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles in {IDENTIFIER} and not in {LITERAL}<br>	∧ +1.reserved = =<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 406.` |
| 7 | `  •••start_col ≥ 9<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER, LITERAL}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 2219.` |
| 8 | `  -1.reserved = ;<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.997. Support: 776.` |
| 9 | `  -1.reserved = ;<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {BLOCK}<br>⇒ y = ⏎<br>Confidence: 0.994. Support: 1848.` |
| 10 | `  -1.reserved = {<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.984. Support: 911.` |
| 11 | `  -1.reserved not in {;, {}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.959. Support: 1229.` |
| 12 | `  -1.reserved not in {;, {}<br>	∧ -1.roles in {EXPRESSION, STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = '<br>Confidence: 0.998. Support: 317.` |
| 13 | `  -1.reserved not in {;, {}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 4768.` |
| 14 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.962. Support: 1818.` |
| 15 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1613.` |
| 16 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles in {FILE} and not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 503.` |
| 17 | `  -1.diff_col ≥ 9<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +3.reserved = .<br>	∧ ^1.roles in {FILE} and not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 167.` |
| 18 | `  -1.diff_col ≥ 9<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +3.reserved not in {.}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {FILE} and not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎<br>Confidence: 0.959. Support: 495.` |
| 19 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {FILE, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 197.` |
| 20 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {FILE, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.977. Support: 152.` |
| 21 | `  -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {)}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {LITERAL} and not in {FILE, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎<br>Confidence: 0.996. Support: 131.` |
| 22 | `  -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {)}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {SCOPE} and not in {FILE, LITERAL, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎<br>Confidence: 0.970. Support: 186.` |
| 23 | `  -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ;, }}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {FILE, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.948. Support: 3072.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.782608695652174, "max_conf": 0.9996899962425232, "max_support": 9618, "min_conf": 0.94775390625, "min_support": 107, "num_rules": 23}}
```
</details>
