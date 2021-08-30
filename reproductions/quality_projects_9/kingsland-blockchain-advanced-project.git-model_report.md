# Model report for file:///tmp/top-repos-quality-repos-conxs3vr/kingsland-blockchain-advanced-project.git HEAD 30c7ea23a73509b0abee2d47920032f112a205c3

### Dump

```json
{'created_at': '2021-08-29 13:21:29',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.4.0-74-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '17.6 kB',
 'tags': [],
 'uuid': '07cda5a4-f601-4371-b49e-4ab68b345f80',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-conxs3vr/kingsland-blockchain-advanced-project.git 30c7ea23a73509b0abee2d47920032f112a205c3

# javascript
25 rules, avg.len. 7.4
## train
PPCR: 0.908162
### report
macro
{'f1-score': 0.8912488343262429,
 'precision': 0.9287293054065018,
 'recall': 0.8739358449556028,
 'support': 34680}
micro
{'f1-score': 0.9741061130334486,
 'precision': 0.9741061130334486,
 'recall': 0.9741061130334486,
 'support': 34680}
weighted
{'f1-score': 0.9732499341193397,
 'precision': 0.9739722950163107,
 'recall': 0.9741061130334486,
 'support': 34680}
### report_full
macro
{'f1-score': 0.7559418665617393,
 'precision': 0.9287293054065018,
 'recall': 0.6842073357817298,
 'support': 38187}
micro
{'f1-score': 0.9272235717128468,
 'precision': 0.9741061130334486,
 'recall': 0.8846466074842224,
 'support': 38187}
weighted
{'f1-score': 0.9149212357377299,
 'precision': 0.9692493110892415,
 'recall': 0.8846466074842224,
 'support': 38187}
## test
PPCR: 0.879322
### report
macro
{'f1-score': 0.5937839293672229,
 'precision': 0.6925204433396619,
 'recall': 0.6316267509727155,
 'support': 7578}
micro
{'f1-score': 0.8308260754816575,
 'precision': 0.8308260754816574,
 'recall': 0.8308260754816574,
 'support': 7578}
weighted
{'f1-score': 0.825361830243655,
 'precision': 0.8603203989460725,
 'recall': 0.8308260754816574,
 'support': 7578}
### report_full
macro
{'f1-score': 0.5021953606083922,
 'precision': 0.6925204433396619,
 'recall': 0.49232891264626655,
 'support': 8618}
micro
{'f1-score': 0.777475919980242,
 'precision': 0.8308260754816574,
 'recall': 0.7305639359480158,
 'support': 8618}
weighted
{'f1-score': 0.7560729196644427,
 'precision': 0.8386831276313866,
 'recall': 0.7305639359480158,
 'support': 8618}
```

## javascript
### Summary
17 rules, avg.len. 7.4

| | |
|-|-|
|Min support|92|
|Max support|6818|
|Min confidence|0.929347813129425|
|Max confidence|0.999765932559967|

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
| 1 | `  -1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 6818.` |
| 2 | `  -1.label not in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = ]<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 107.` |
| 3 | `  -1.label not in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {]}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.941. Support: 2999.` |
| 4 | `  -1.internal_type = Identifier<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 654.` |
| 5 | `  -1.internal_type = Identifier<br>	∧ +1.reserved not in {=}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 4204.` |
| 6 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 2136.` |
| 7 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.929. Support: 92.` |
| 8 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 1483.` |
| 9 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = {<br>	∧ +1.reserved not in {;}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.989. Support: 874.` |
| 10 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.947. Support: 837.` |
| 11 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved not in {;, if, }}<br>	∧ +5.reserved = function<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.959. Support: 110.` |
| 12 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 415.` |
| 13 | `  -1.diff_offset ≥ 103<br>	∧ -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.label not in {<newline>}<br>	∧ +1.reserved not in {;, }}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 113.` |
| 14 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 352.` |
| 15 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.reserved not in {), ;, }}<br>	∧ +1.roles in {KEY}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.978. Support: 297.` |
| 16 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 156.` |
| 17 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.966. Support: 4184.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 7.352941176470588, "max_conf": 0.999765932559967, "max_support": 6818, "min_conf": 0.929347813129425, "min_support": 92, "num_rules": 17}}
```
</details>
