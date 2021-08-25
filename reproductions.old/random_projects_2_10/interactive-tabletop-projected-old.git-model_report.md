# Model report for file:///tmp/top-repos-quality-repos-syesqkv9/interactive-tabletop-projected-old.git HEAD 9456d214c4116020ae10bd093fee88e3d5a897a4

### Dump

```json
{'created_at': '2021-08-22 06:29:16',
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
 'size': '16.5 kB',
 'tags': [],
 'uuid': 'f40010bb-3e81-4000-8ce9-2ae6bf9295fb',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-syesqkv9/interactive-tabletop-projected-old.git 9456d214c4116020ae10bd093fee88e3d5a897a4

# javascript
47 rules, avg.len. 4.9
## train
PPCR: 0.972133
### report
macro
{'f1-score': 0.6292154426224487,
 'precision': 0.6428238959869441,
 'recall': 0.6177837010829427,
 'support': 10535}
micro
{'f1-score': 0.9161841480778358,
 'precision': 0.9161841480778358,
 'recall': 0.9161841480778358,
 'support': 10535}
weighted
{'f1-score': 0.9058453454670754,
 'precision': 0.8965139477033727,
 'recall': 0.9161841480778358,
 'support': 10535}
### report_full
macro
{'f1-score': 0.6148145560255688,
 'precision': 0.6428238959869441,
 'recall': 0.5910031817184604,
 'support': 10837}
micro
{'f1-score': 0.9032378813400711,
 'precision': 0.9161841480778358,
 'recall': 0.8906523945741441,
 'support': 10837}
weighted
{'f1-score': 0.89057298184947,
 'precision': 0.8917399621081011,
 'recall': 0.8906523945741441,
 'support': 10837}
## test
PPCR: 0.947103
### report
macro
{'f1-score': 0.5018650586444753,
 'precision': 0.534953065004191,
 'recall': 0.4861446545657072,
 'support': 376}
micro
{'f1-score': 0.8191489361702128,
 'precision': 0.8191489361702128,
 'recall': 0.8191489361702128,
 'support': 376}
weighted
{'f1-score': 0.783187407307768,
 'precision': 0.7574259233749386,
 'recall': 0.8191489361702128,
 'support': 376}
### report_full
macro
{'f1-score': 0.49659453015292226,
 'precision': 0.534953065004191,
 'recall': 0.4744722918407129,
 'support': 397}
micro
{'f1-score': 0.796895213454075,
 'precision': 0.8191489361702128,
 'recall': 0.7758186397984886,
 'support': 397}
weighted
{'f1-score': 0.7582335051352579,
 'precision': 0.7479641412294333,
 'recall': 0.7758186397984886,
 'support': 397}
```

## javascript
### Summary
28 rules, avg.len. 4.8

| | |
|-|-|
|Min support|140|
|Max support|2636|
|Min confidence|0.9210526347160339|
|Max confidence|0.9985207319259644|

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
                     'base_model_name': 'sklearn.ensemble.RandomForestClassifier',
                     'confidence_threshold': 0.8,
                     'min_samples_leaf': 90,
                     'min_samples_split': 180,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -2.diff_col ≥ 9<br>	∧ -3.diff_col ≥ 11<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.955. Support: 257.` |
| 2 | `  -2.diff_col ≤ 8<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.927. Support: 1246.` |
| 3 | `  ^1.roles in {QUALIFIED} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 2636.` |
| 4 | `  -1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IF, OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.928. Support: 1291.` |
| 5 | `  -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.975. Support: 141.` |
| 6 | `  -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.948. Support: 182.` |
| 7 | `  -1.reserved not in {;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 330.` |
| 8 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type = Identifier<br>	∧ +1.reserved not in {;}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.935. Support: 424.` |
| 9 | `  -2.diff_offset ≥ 9<br>	∧ -3.diff_offset ≥ 11<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.949. Support: 267.` |
| 10 | `  -2.diff_offset ≥ 9<br>	∧ -3.diff_offset ≤ 10<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.956. Support: 149.` |
| 11 | `  ^1.roles in {IDENTIFIER} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 2620.` |
| 12 | `  -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.959. Support: 185.` |
| 13 | `  -1.reserved not in {;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 329.` |
| 14 | `  •••start_col ≥ 8<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;}<br>	∧ ^1.roles in {DECLARATION} and not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.976. Support: 269.` |
| 15 | `  -3.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.974. Support: 940.` |
| 16 | `  -1.roles in {BINARY}<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 327.` |
| 17 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 311.` |
| 18 | `  -1.roles in {LEFT}<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 325.` |
| 19 | `  -1.reserved not in {,, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = {<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.982. Support: 192.` |
| 20 | `  -2.diff_offset ≥ 9<br>	∧ +1.reserved = =<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 238.` |
| 21 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 338.` |
| 22 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {;}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.922. Support: 458.` |
| 23 | `  -1.reserved = {<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.925. Support: 140.` |
| 24 | `  +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.962. Support: 172.` |
| 25 | `  -2.diff_offset ≥ 9<br>	∧ -4.diff_offset ≥ 13<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.968. Support: 262.` |
| 26 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 274.` |
| 27 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 3<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.921. Support: 171.` |
| 28 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.992. Support: 178.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.785714285714286, "max_conf": 0.9985207319259644, "max_support": 2636, "min_conf": 0.9210526347160339, "min_support": 140, "num_rules": 28}}
```
</details>
