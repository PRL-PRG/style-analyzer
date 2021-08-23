# Model report for file:///tmp/top-repos-quality-repos-0f6eizwl/escape_trumps_america.git HEAD b91c31830427f88aa2254256d5453bd4dc1024c7

### Dump

```json
{'created_at': '2021-08-22 04:45:49',
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
 'size': '17.8 kB',
 'tags': [],
 'uuid': '208ebd92-9b74-477c-92f9-140fa5f4a535',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-0f6eizwl/escape_trumps_america.git b91c31830427f88aa2254256d5453bd4dc1024c7

# javascript
72 rules, avg.len. 5.0
## train
PPCR: 0.954278
### report
macro
{'f1-score': 0.5857546926012647,
 'precision': 0.5895965562261327,
 'recall': 0.5836941873745154,
 'support': 8432}
micro
{'f1-score': 0.9165085388994307,
 'precision': 0.9165085388994307,
 'recall': 0.9165085388994307,
 'support': 8432}
weighted
{'f1-score': 0.8925924150745871,
 'precision': 0.870725344394035,
 'recall': 0.9165085388994307,
 'support': 8432}
### report_full
macro
{'f1-score': 0.5570956446271702,
 'precision': 0.5895965562261327,
 'recall': 0.5401997447043485,
 'support': 8836}
micro
{'f1-score': 0.8950660180681027,
 'precision': 0.9165085388994307,
 'recall': 0.8746038931643277,
 'support': 8836}
weighted
{'f1-score': 0.864708016055539,
 'precision': 0.8618844205637733,
 'recall': 0.8746038931643277,
 'support': 8836}
## test
PPCR: 0.954926
### report
macro
{'f1-score': 0.5898366495372362,
 'precision': 0.5933624791299871,
 'recall': 0.5877828962062254,
 'support': 8220}
micro
{'f1-score': 0.917396593673966,
 'precision': 0.917396593673966,
 'recall': 0.917396593673966,
 'support': 8220}
weighted
{'f1-score': 0.8933122451902439,
 'precision': 0.8712081182242767,
 'recall': 0.917396593673966,
 'support': 8220}
### report_full
macro
{'f1-score': 0.5605766977082491,
 'precision': 0.5933624791299871,
 'recall': 0.5426513149058285,
 'support': 8608}
micro
{'f1-score': 0.8962443546470169,
 'precision': 0.917396593673966,
 'recall': 0.8760455390334573,
 'support': 8608}
weighted
{'f1-score': 0.8662017882165503,
 'precision': 0.863017135899017,
 'recall': 0.8760455390334573,
 'support': 8608}
```

## javascript
### Summary
50 rules, avg.len. 4.5

| | |
|-|-|
|Min support|148|
|Max support|2136|
|Min confidence|0.9217361807823181|
|Max confidence|0.9978070259094238|

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
| 1 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 2052.` |
| 2 | `  -3.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.951. Support: 618.` |
| 3 | `  -1.reserved = {<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.998. Support: 228.` |
| 4 | `  -1.reserved = (<br>	∧ +1.roles in {IDENTIFIER}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 175.` |
| 5 | `  +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.973. Support: 242.` |
| 6 | `  +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.964. Support: 238.` |
| 7 | `  -3.length ≥ 4<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.958. Support: 633.` |
| 8 | `  +1.reserved = }<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.961. Support: 270.` |
| 9 | `  -1.reserved = {<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.988. Support: 216.` |
| 10 | `  -1.reserved = (<br>	∧ +1.internal_type = Identifier<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 174.` |
| 11 | `  -1.reserved = {<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.969. Support: 273.` |
| 12 | `  -1.reserved = ;<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.981. Support: 184.` |
| 13 | `  -1.reserved not in {;, {}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.960. Support: 645.` |
| 14 | `  -1.reserved not in {;, {}<br>	∧ -1.roles in {BINARY}<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.936. Support: 148.` |
| 15 | `  -1.reserved not in {;, {}<br>	∧ +1.reserved = {<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.958. Support: 274.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -3.diff_line = 0<br>	∧ +1.reserved not in {{}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.922. Support: 1463.` |
| 17 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 2136.` |
| 18 | `  -3.length ≥ 4<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.956. Support: 604.` |
| 19 | `  +1.reserved = }<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.974. Support: 250.` |
| 20 | `  -1.reserved = {<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.989. Support: 218.` |
| 21 | `  -1.reserved = (<br>	∧ -4.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.926. Support: 196.` |
| 22 | `  +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.940. Support: 207.` |
| 23 | `  -1.reserved = (<br>	∧ -4.internal_type not in {Identifier}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.960. Support: 188.` |
| 24 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.995. Support: 2132.` |
| 25 | `  -3.length ≥ 3<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.964. Support: 620.` |
| 26 | `  -1.reserved = ;<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.970. Support: 183.` |
| 27 | `  -1.reserved = {<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.980. Support: 225.` |
| 28 | `  -1.internal_type = Identifier<br>	∧ -1.reserved not in {;, {}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.942. Support: 784.` |
| 29 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {;, {}<br>	∧ +1.reserved = ;<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.971. Support: 391.` |
| 30 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.length ≥ 3<br>	∧ +1.reserved not in {), ;}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.927. Support: 592.` |
| 31 | `  -4.diff_offset ≥ 12<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.933. Support: 699.` |
| 32 | `  -1.internal_type = Identifier<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.951. Support: 806.` |
| 33 | `  -1.internal_type not in {Identifier}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.966. Support: 252.` |
| 34 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = {<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.984. Support: 221.` |
| 35 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.964. Support: 181.` |
| 36 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {;, {}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.958. Support: 418.` |
| 37 | `  -4.diff_offset ≥ 12<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.937. Support: 710.` |
| 38 | `  -1.internal_type = Identifier<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.956. Support: 835.` |
| 39 | `  -1.internal_type not in {Identifier}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.986. Support: 249.` |
| 40 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = {<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.975. Support: 219.` |
| 41 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {;, {}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 373.` |
| 42 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = (<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {CALL}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.943. Support: 344.` |
| 43 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.length ≥ 3<br>	∧ +1.reserved not in {), ;, }}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.956. Support: 306.` |
| 44 | `  -2.diff_offset ≥ 9<br>	∧ -4.diff_offset ≤ 11<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.930. Support: 149.` |
| 45 | `  -1.reserved = ;<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.981. Support: 183.` |
| 46 | `  -1.reserved = ;<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {IDENTIFIER}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.944. Support: 205.` |
| 47 | `  -1.reserved = {<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.980. Support: 220.` |
| 48 | `  -1.internal_type = Identifier<br>	∧ -1.reserved not in {;, {}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.952. Support: 816.` |
| 49 | `  -1.reserved = (<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {IDENTIFIER}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 160.` |
| 50 | `  •••start_col ≥ 21<br>	∧ -1.roles not in {LITERAL}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.952. Support: 1156.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.5, "max_conf": 0.9978070259094238, "max_support": 2136, "min_conf": 0.9217361807823181, "min_support": 148, "num_rules": 50}}
```
</details>
